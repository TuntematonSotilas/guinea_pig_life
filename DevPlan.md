# Guinea Pig Life — Plan de développement

> Document technique de référence. Le design du jeu est décrit dans [GamePlay.md](GamePlay.md).
> Cible : **Android**, écran **portrait**, **Godot 4.7** (renderer Mobile) + **GDScript**, pixel art style Game Boy Color, EN/FR/DE/ES.

---

## 1. Affichage portrait et pixel art

| Réglage (`project.godot`) | Valeur | Statut |
|---|---|---|
| `display/window/size/viewport_width` × `viewport_height` | **180 × 320** (9:16, ×6 = 1080×1920) | ✅ |
| `display/window/size/window_width_override` × `height_override` | 540 × 960 (test dans l'éditeur) | ✅ |
| `display/window/handheld/orientation` | Portrait (1) | ✅ |
| `display/window/stretch/mode` / `aspect` / `scale_mode` | `viewport` / **`keep_width`** / `integer` | ✅ |
| `rendering/textures/canvas_textures/default_texture_filter` | Nearest | ✅ |
| `rendering/2d/snap/snap_2d_transforms_to_pixel` + `vertices` | true | ✅ |
| `input_devices/pointing/emulate_touch_from_mouse` | true (tester le tactile à la souris) | ✅ |
| `application/boot_splash/bg_color` | `#FAF3F0` (crème de la palette) | ⏳ à corriger |
| `gui/theme/custom` | `res://assets/ui/cosy_theme.tres` | ⏳ |

- La largeur est fixe (180 px). Sur les téléphones plus allongés (19.5:9, 20:9), la hauteur s'allonge : par exemple, 1080×2400 donne 180×400.
- **Tuiles de 16×16**. Une pièce fait environ **11 tuiles de large sur 20 à 30 de haut** (« tranches verticales »). La `Camera2D` suit le cochon, avec des limites calées sur les bords de la pièce.
- Le HUD tient compte de la **safe area**, via `DisplayServer.get_display_safe_area()`.

**Layout écran (jouable à une main) :**
```
┌──────────────┐  ← safe area
│ HUD jauges   │  Food / Water / Health / Happiness + heure (soleil/lune) + XP
├──────────────┤
│  VUE JEU     │  Camera2D qui suit le cochon
│  (pièce)     │
├──────────────┤
│ [Weeek][👃]  │  boutons d'action (zone du pouce)
│ [Quêtes][☰]  │
└──────────────┘
```

---

## 2. Architecture du projet

```
res://
├── autoload/                 # singletons (Project Settings > Autoload)
│   ├── event_bus.gd          ✅ signaux globaux
│   ├── settings.gd           ✅ langue, volumes, vibration
│   ├── palette.gd            ⏳ constantes Color de la palette Cosy
│   ├── save_manager.gd       ⏳ seule couche SQLite
│   ├── game_clock.gd         ⏳ heure du jeu, phases de la journée
│   ├── player_stats.gd       ⏳ jauges, XP, niveau
│   ├── quest_manager.gd      ⏳ quêtes du jour + histoire
│   ├── friendship.gd         ⏳ amitié par animal (Pipou, Minou, Robby, Cui-Cui, Bulle)
│   ├── inventory.gd          ⏳ légumes, graines, trésors, déco
│   └── scene_loader.gd       ⏳ changement de pièce asynchrone + fondu
├── scenes/
│   ├── boot/                 # Boot (loader) → MainMenu
│   ├── ui/                   # MainMenu, HUD, QuestBoard, Inventory, Pause, DaySummary, Dialogue
│   ├── actors/               # GuineaPig, Companion, Npc (base), Mouse, Cat, Robby, Bird, Fish, Human
│   ├── props/                # Interactable (base), Bowl, WaterDispenser, Feeder, HayRack, GardenPlot, Litter,
│   │                         # Poop, Treasure, DecoSlot, Gate, Door, Toy
│   ├── rooms/                # PigRoom, Bedroom, Kitchen, LivingRoom
│   └── minigames/            # NoseFoot, HideCarrot (v1) ; TunnelRun, WheekRhythm (plus tard)
├── data/
│   ├── quests/*.tres         # QuestData
│   ├── items/*.tres          # ItemData (légumes, graines, trésors, déco)
│   ├── skins/*.tres          # SkinData
│   ├── npcs/*.tres           # NpcData (nom, dialogues, cadeaux aimés)
│   └── i18n/translations.csv ✅
├── assets/  sprites/ tiles/ ui/ fonts/ sfx/ music/
├── design/                   # sources graphiques (gp.png, .aseprite)
└── addons/godot-sqlite/      # à installer via l'AssetLib
```

**Principes :**
- **Les données sont décrites dans des Resources** (`.tres`) : quêtes, objets, skins et PNJ. On ajoute du contenu sans toucher au code.
- **Les systèmes communiquent par `EventBus`**, sans référence directe entre eux.
- **Une classe de base `Interactable`** (`Area2D`) porte une icône, une action et un point d'approche. Toucher l'objet fait venir le cochon, puis déclenche `interact(pig)`. Tous les objets et PNJ en héritent.
- **`SaveManager` est la seule classe qui parle à SQLite.** Les autres systèmes exposent `to_dict()` / `from_dict()`.

---

## 3. Systèmes

### 3.1 Le cochon (`actors/guinea_pig.tscn`)
- `CharacterBody2D` + `AnimatedSprite2D` + `NavigationAgent2D` + collision d'environ 16×8 px aux pieds.
- **Machine à états** : `Idle`, `Walk`, `Eat`, `Drink`, `Squeak`, `NosePunch`, `Poop`, `Sleep`, `Popcorn`.
- **Sprites** : cellules de 32×32, une ligne par animation. Il y a 3 directions (face, dos, profil) ; le côté droit s'obtient en retournant le profil (`flip_h`).
  Le sprite existant, `design/gp.png`, est l'animation d'attente de profil (2 frames).
- **Poop** : se déclenche automatiquement un peu après avoir mangé. La crotte (`Poop`) est un objet qu'on pousse d'un coup de nez.
- **Popcorning** : se déclenche quand Happiness > 80. Il active un multiplicateur d'XP ×1,5 dans `PlayerStats`.

### 3.2 Contrôles tactiles
- **Toucher le sol** : le cochon s'y rend (pathfinding sur `NavigationRegion2D`, construite à partir des `TileMapLayer`).
- **Toucher un `Interactable`** : le cochon s'en approche, puis fait l'action automatiquement.
- **Boutons** :
  - **Weeek** : appel, déclenchement du distributeur, sonar la nuit, avec un temps de recharge.
  - **Coup de nez** : pousse l'objet ou le PNJ qui se trouve devant le cochon.
- Un retour haptique (`Input.vibrate_handheld`) accompagne les actions, s'il est activé dans les réglages.

### 3.3 Jauges et progression (`player_stats.gd`)
- Food, Water, Health et Happiness vont de 0 à 100 et baissent à chaque minute de jeu, avec des taux réglables dans une Resource `Balance.tres`.
  - Health baisse pour chaque crotte au sol et quand Food ou Water sont à 0.
- **Effets à 0** :
  - Food ou Water : le cochon est ralenti.
  - Health : visite chez le véto (fin de journée, légère perte d'XP).
  - Happiness : pas de popcorning.
- **XP** : `xp_needed(level) = 100 * level^1.5`. Au passage de niveau, `EventBus.leveled_up` déclenche les déblocages (skins, déco, cochons aux niveaux 5, 10 et 15, pièces).

### 3.4 Temps et jour/nuit (`game_clock.gd`)
- 1 journée de jeu ≈ **12 à 15 min réelles** (valeur réglable).
- **Phases** :
  - Matin : 6h–9h.
  - Journée : 9h–18h.
  - Soir : 18h–21h (**contenu à définir**).
  - Nuit : 21h–6h.
- **Signaux** : `minute_tick`, `hour_changed`, `phase_changed`, `day_ended`.
- **Rendu** : un `CanvasModulate` suit un `Gradient` (la nuit est teintée lavande foncé `#6B5A8E`).
  La nuit, des `PointLight2D` éclairent les lampes et la lune, avec un halo de vision autour du cochon.
- **Hors-ligne** : au lancement, on calcule le temps écoulé depuis la dernière sauvegarde et on applique la décroissance, plafonnée à −50 %.

### 3.5 La PigRoom (la pièce de base)
- **Maisonnette** : dormir termine la journée, puis affiche l'écran `DaySummary` et lance la sauvegarde.
- **Râtelier à foin** : Food illimité, mais peu nourrissant.
- **Distributeur d'eau** : se recharge chaque matin.
- **Distributeur de granulés** : se déclenche à heures fixes, ou plus tôt avec un Weeek à proximité.
- **Potager** : des parcelles `GardenPlot` passent par les états vide → planté → arrosé → pousse → mûr. La pousse avance en heures de jeu.
- **Litière** : pousser une crotte dedans la supprime et remonte Health.
- **Emplacements de déco** : on y place des trésors ou des objets de déco, sauvegardés par emplacement.
- **Tableau de quêtes** et **portillon** vers la Bedroom (ouvert d'un coup de nez).

### 3.6 Les PNJ animaux (`friendship.gd` + `actors/npc`)
- Une classe de base `Npc` (qui hérite d'`Interactable`) gère une bulle de dialogue, les cadeaux (les objets aimés sont définis dans `NpcData`) et une amitié de 0 à 5 cœurs.
- **Pipou** (tutoriel), **Minou** (rival qui devient ami, avec une IA qui bloque des passages ou vole des légumes), **Robby** (cassé, puis réparé : il nettoie une pièce par jour), **Cui-Cui** (graines rares, météo), **Bulle** (énigmes, quêtes trésor).
- **IA** : simple, sur horaires (`GameClock`) et avec quelques points de patrouille. Pas de pathfinding complexe en v1.

### 3.7 Les humains (en fond)
- Des PNJ `Human` non interactifs, qui traversent une pièce à certaines heures, avec peu d'animations (marche et une pose).
- Le matin, un Weeek au bon moment fait tomber un légume en bonus.

### 3.8 Nuit : exploration
- Les **trésors** (`Treasure`) sont cachés dans des zones sombres. Le **sonar Weeek** les révèle 2 à 3 s (shader d'outline, ou sprite qui clignote).
- Minou peut dormir sur le chemin. Si on fait trop de bruit près de lui, il se réveille et bloque le passage.

### 3.9 Quêtes (`quest_manager.gd`)
- `QuestData` contient : `id`, `title_key`, `desc_key`, `type` (`harvest`, `clean_poop`, `give_item`, `find_treasure`, `visit_room`, `talk_to`…), `target_id`, `count`, `xp_reward`, `item_reward`, `giver` (tableau ou PNJ), `is_story`, `prerequisites`.
- **Quêtes du jour** : 3 tirées au hasard à chaque `day_started`.
- **Quêtes histoire** : chaînées par `prerequisites`.
- **Progression** : le manager écoute les signaux d'`EventBus` (`poop_cleaned`, `item_harvested`…) et fait avancer les compteurs automatiquement.

### 3.10 Mini-jeux (pour la Happiness)
- Ce sont des scènes séparées, lancées depuis la LivingRoom par `SceneLoader`. Elles renvoient un score, converti en Happiness et en XP.
- **v1** : **Nose Foot** (physique `RigidBody2D` sur une balle) et **Cache-carotte** (chrono + objets cachés).
- **Plus tard** : Tunnel Run et Weeek Rhythm.

### 3.11 UI et i18n
- **Thème** `cosy_theme.tres` : `StyleBoxFlat` avec fond crème, bordure encre de 1 px, coins de 2 px et ombre de 1 px. `palette.gd` reprend les mêmes couleurs pour le code.
- **Écrans** : MainMenu (New / Continue / Langue / Réglages), HUD, QuestBoard, Inventaire, Dialogue, DaySummary et Pause.
- **Traductions** : un fichier `translations.csv` (clé, en, fr, de, es), `tr()` partout, et aucun texte en dur.
  Au premier lancement, la langue est celle du téléphone (`settings.gd` ✅).
- **Police** : une police pixel qui gère les accents FR/DE/ES, le ß, le ñ et les signes ¿ ¡.

---

## 4. Sauvegarde SQLite

Elle repose sur le plugin **godot-sqlite** (2shady4u, GDExtension compatible Android). La base est stockée dans `user://save.db`.

```sql
CREATE TABLE meta          (key TEXT PRIMARY KEY, value TEXT);            -- schema_version
CREATE TABLE settings      (key TEXT PRIMARY KEY, value TEXT);
CREATE TABLE save_slot     (id INTEGER PRIMARY KEY, created_at TEXT, updated_at TEXT,
                            game_day INTEGER, game_minute INTEGER, current_room TEXT,
                            active_pig_id INTEGER, xp INTEGER, level INTEGER, last_real_time INTEGER);
CREATE TABLE pig           (id INTEGER PRIMARY KEY, slot_id INTEGER, name TEXT, skin_id TEXT,
                            food REAL, water REAL, health REAL, happiness REAL, pos_x REAL, pos_y REAL);
CREATE TABLE inventory     (slot_id INTEGER, item_id TEXT, qty INTEGER, PRIMARY KEY(slot_id, item_id));
CREATE TABLE garden_plot   (slot_id INTEGER, plot_id TEXT, seed_id TEXT, state INTEGER,
                            planted_minute INTEGER, watered INTEGER, PRIMARY KEY(slot_id, plot_id));
CREATE TABLE deco_slot     (slot_id INTEGER, deco_slot_id TEXT, item_id TEXT, PRIMARY KEY(slot_id, deco_slot_id));
CREATE TABLE friendship    (slot_id INTEGER, npc_id TEXT, hearts REAL, flags TEXT, PRIMARY KEY(slot_id, npc_id));
CREATE TABLE quest_progress(slot_id INTEGER, quest_id TEXT, progress INTEGER, completed INTEGER,
                            PRIMARY KEY(slot_id, quest_id));
CREATE TABLE unlock        (slot_id INTEGER, kind TEXT, item_id TEXT);   -- skins, pigs, rooms
CREATE TABLE room_state    (slot_id INTEGER, room TEXT, state_json TEXT); -- crottes, trésors ramassés
```

- **Sauvegarde automatique** : quand le cochon s'endort (fin de journée), au changement de pièce, toutes les 60 s, et sur `NOTIFICATION_APPLICATION_PAUSED` (quand Android met l'app en arrière-plan).
- **Migrations** : `meta.schema_version` indique la version du schéma, et une fonction `migrate()` passe de la version N à N+1.
- **Sans plugin** : `SaveManager` vérifie `ClassDB.class_exists("SQLite")`. Si le plugin est absent, il se replie sur un fichier JSON pour le développement.

---

## 5. Android et splash screen

1. **Éditeur Godot** (`Editor Settings > Export > Android`) : indiquer le chemin du **SDK** (celui d'Android Studio) et d'un **JDK 17**.
2. **Templates** : installer les *Export Templates* 4.7, puis lancer `Project > Install Android Build Template` pour activer le **Gradle build**.
3. **Preset Android** : orientation portrait, `min_sdk 24`, `target_sdk` à la valeur exigée par le Play Store. Désactiver les permissions inutiles.
4. **Keystores** : un keystore debug pour le déploiement USB en un clic, puis un keystore **release** à sauvegarder hors du dépôt. L'export final se fait en **AAB**.
5. **Splash natif Android 12+** : modifier `android/build/res/values/themes.xml`.
   - `windowSplashScreenBackground` = `#FAF3F0`.
   - `windowSplashScreenAnimatedIcon` = la tête du cochon (vector drawable ou AVD).
   - `postSplashScreenTheme`.
6. **Loader dans le jeu** : `scenes/boot/boot.tscn` affiche le cochon en animation d'attente et une barre de progression, pilotées par `ResourceLoader.load_threaded_request()`. Il enchaîne ensuite sur le MainMenu. Même fond crème que le splash natif, pour une transition invisible.

> ⚠️ `/android/` est dans `.gitignore`. Il faudra en versionner les fichiers personnalisés (`themes.xml`, les drawables), par exemple dans `android_custom/`, et les recopier après chaque réinstallation du template.

---

## 6. Roadmap

Principe : on reste en **placeholders** (formes simples) jusqu'à la fin du **jalon A**, pour trouver le fun avant d'investir dans l'art.

| Étape | Contenu | Livrable |
|---|---|---|
| **0. Setup** *(en cours)* | Réglages portrait/pixel ✅, arborescence ✅, traductions ✅, `event_bus` ✅, `settings` ✅. Reste : `palette.gd`, thème, squelettes des autres autoloads, scène Boot, SpriteFrames de `gp.png`, plugin SQLite, **1er APK sur téléphone** | L'app s'ouvre en portrait sur le téléphone |
| **1. Déplacement** | PigRoom en tiles placeholder, cochon (`gp.png`), touch-to-go + navigation, caméra, HUD et boutons | On se promène dans la PigRoom |
| **2. Soins** | `Interactable`, foin, distributeur d'eau, distributeur de granulés, litière, crottes et coup de nez, jauges + HUD, Weeek | Boucle de soin jouable |
| **3. Temps** | GameClock, phases, jour/nuit, maisonnette + fin de journée + DaySummary | Une journée complète |
| **4. Sauvegarde** | SaveManager SQLite, Continue/New, autosave, hors-ligne | Quitter et reprendre |
| **🎯 Jalon A : vertical slice** | **PigRoom complète et jouable en boucle** | On teste le fun |
| **5. Potager et inventaire** | GardenPlot, graines et légumes, Inventory, effets des légumes | Boucle de ferme |
| **6. Maison** | Bedroom, Kitchen, LivingRoom, portes et portillon, SceneLoader + fondu | Maison complète |
| **7. PNJ et amitié** | Npc de base, dialogues, cadeaux, Pipou (tutoriel), Robby, Minou (rival puis ami), Cui-Cui, Bulle, humains en fond | Communauté vivante |
| **8. Nuit** | Lumières, halo, trésors, sonar Weeek, Minou endormi | Exploration nocturne |
| **9. Quêtes et progression** | QuestManager, tableau de quêtes, quêtes du jour et histoire, XP, déblocages, déco de la PigRoom | Méta-progression |
| **10. Mini-jeux v1** | Nose Foot, Cache-carotte | Happiness via le jeu |
| **🎯 Jalon B : contenu v1** | Tout le gameplay en placeholders améliorés | Démo interne |
| **11. Art et audio** | Sprites Aseprite (cochon 3 directions, animaux, décors), tilesets, UI pixel, sons (Weeek !), musique chiptune jour/nuit | Style final |
| **12. Menu, i18n, polish** | MainMenu animé, traductions complètes EN/FR/DE/ES, police, splash natif + loader, effets visuels, vibrations | Première impression |
| **13. Skins et multi-cochons** | Races, accessoires, cochons compagnons qui suivent | Récompenses long terme |
| **14. Release** | Tests sur plusieurs appareils, performances, keystore release, AAB, fiche Play Store dans les 4 langues | Publication |

---

## 7. Prochaines actions (fin de l'étape 0)

1. Corriger `boot_splash/bg_color` (`#FAF3F0`) et ajouter `gui/theme/custom` dans `project.godot`.
2. Créer `autoload/palette.gd` et `assets/ui/cosy_theme.tres`.
3. Créer les squelettes `save_manager.gd`, `game_clock.gd`, `player_stats.gd`, `quest_manager.gd`, `friendship.gd`, `inventory.gd` et `scene_loader.gd`, puis les enregistrer comme autoloads.
4. Créer `assets/sprites/guinea_pig/gp_frames.tres` (SpriteFrames, animation `idle_side` de 2 frames à 2 fps) et `scenes/boot/boot.tscn`.
5. **Toi** : installer godot-sqlite via l'AssetLib, configurer le SDK et le JDK, puis lancer le premier export sur le téléphone.

## 8. Vérification

- **Dans l'éditeur (F5)** : une fenêtre portrait de 540×960, des pixels nets, et le texte du Boot traduit selon la langue.
- **Hauteur variable** : avec un window override de 360×800, la largeur reste 180 et la hauteur s'allonge.
- **Sur le téléphone (debug USB)** : l'app est verrouillée en portrait, avec le splash crème puis l'écran Boot.

## 9. Décisions ouvertes (voir [GamePlay.md](GamePlay.md))

- Le contenu du **soir** (18h–21h), qui remplace les miettes et le câlin.
- La durée exacte d'une journée et la décroissance hors-ligne.
- Le casting des animaux pour la v1.
- Les cochons compagnons : ensemble dans la maison, ou un seul par partie.
