# Guinea Pig Life — Game Design (GamePlay)

> Document de référence du gameplay. Écran **portrait**, pixel art style Game Boy Color, Godot 4.7, Android, EN/FR/DE/ES.

## Pitch
*Guinea Pig Life* est un jeu cozy : tu es un petit cochon d'inde qui vit dans une maison. Une famille humaine y habite,
mais elle est **souvent absente** (travail, école…). Tu te débrouilles donc surtout seul, avec tes **amis animaux** de la maison.
Tu fais pousser tes légumes, tu explores, tu trouves des trésors. **Le jour** tu prends soin de toi et de ta PigRoom ;
**la nuit** la maison t'appartient. Pas de game over : on joue pour rendre son cochon heureux et agrandir sa petite famille.

> **Direction retenue : HYBRIDE.** Les humains sont **en fond** : on les voit passer de temps en temps, mais ils ne sont pas
> au cœur du gameplay. L'essentiel repose sur l'**autonomie** (distributeur, potager, raids) et la **communauté d'animaux**.

## Boucle de jeu
- **Boucle courte (1–3 min)** : regarder ses jauges → aller manger/boire/jouer/nettoyer → faire une action → gagner de l'XP.
- **Boucle moyenne (1 journée de jeu ≈ 12–15 min)** : 3 quêtes du jour, le potager, les visites des humains, la nuit d'exploration, puis dodo = sauvegarde et bilan de la journée.
- **Boucle longue** : level up → skins, accessoires, nouveaux cochons, déco de la PigRoom, nouvelles zones.

## La journée type
| Moment | Ce qui se passe | Gameplay |
|---|---|---|
| Matin (6h–9h) | Les humains passent en coup de vent avant de partir, bruit du frigo | Un **Weeek !** au bon moment : un humain jette un légume en passant (petit bonus). Le **distributeur** de la PigRoom se déclenche. |
| Journée (9h–18h) | La maison est vide, à toi ! | **Potager** (arroser, récolter), ménage de la PigRoom, visites aux **amis animaux**, mini-jeux, quêtes |
| Soir (18h–21h) | La famille rentre et dîne | **Événement de fond** : des miettes tombent de la table (bonus Food si tu es rapide). L'enfant passe parfois faire un câlin (bonus Happiness aléatoire). |
| Nuit (21h–6h) | Tout le monde dort, lumière tamisée | **Exploration** : raid dans la cuisine, trésors cachés, zones sombres (halo de vision autour du cochon), **Weeek sonar** — attention à ne pas réveiller Minou |

## Les actions du cochon
- **Weeek (cri)** : appeler ses amis animaux, activer le distributeur plus tôt, attirer un humain qui passe.
  La nuit, c'est un **sonar** : il révèle un court instant les trésors cachés dans le noir.
  Clin d'œil réaliste : un bruit de sachet plastique ou de frigo → le cochon fait Weeek tout seul.
  Un temps de recharge court évite le spam.
- **Nose punch (coup de nez)** : pousser des objets (balle, carotte, crotte, carton), faire tomber un légume d'une
  table basse, ouvrir le portillon ou une porte entrouverte, faire fuir Minou (tant qu'il est encore rival).
- **Manger / Boire** : aux gamelles, au distributeur d'eau, au foin, avec les légumes trouvés (chaque légume a son effet).
- **Poop** : automatique après avoir mangé (animation drôle). Les crottes baissent **Health** si on ne les pousse pas
  dans la litière (nose punch). Une fois réparé, **Robby l'aspirateur robot** en ramasse aussi dans une pièce par jour.
- **Popcorning** (vrai comportement des cochons d'inde) : quand Happiness > 80, le cochon fait des petits bonds de joie
  → **bonus d'XP ×1,5** pendant que ça dure. C'est la récompense visuelle d'un cochon bien soigné.
- **Dormir** : dans la maisonnette de sa pièce dédiée (**PigRoom**). Ça remonte Health et termine la journée
  (bilan + sauvegarde). Si la nuit se termine loin de la PigRoom, le cochon s'endort sur place : moins de Health récupérée.

## Les jauges
| Jauge | Baisse | Remonte | À 0 |
|---|---|---|---|
| Food | avec le temps | légumes, foin, granulés | le cochon est affaibli (plus lent) |
| Water | avec le temps (plus vite le jour) | distributeur d'eau, fontaine | le cochon est affaibli |
| Health | crottes au sol, Food/Water à 0 | ménage, sommeil, vitamine C (poivron, orange) | **visite chez le véto** : la journée s'arrête, perte d'un peu d'XP (pas de game over) |
| Happiness | ennui, solitude | jouets, mini-jeux, visites aux amis animaux, câlin surprise de l'enfant, amis cochons | le cochon boude (pas de popcorning, XP réduite) |

## Les pièces
- **PigRoom** (nouvelle pièce, point de départ et base du joueur) : une pièce entière transformée en
  **grand enclos** pour cochon d'inde. C'est une cage géante qui occupe toute la pièce, avec des barrières sur les bords.
  On y trouve :
  - la **maisonnette/igloo** (le cochon y dort, ce qui termine la journée et sauvegarde) ;
  - le **râtelier à foin** (Food illimité mais peu nourrissant) ;
  - le **distributeur d'eau** (Water, qui se recharge chaque matin) et le **distributeur automatique** de granulés (heures fixes, ou déclenché par un Weeek) ;
  - le **potager** : des bacs où planter, arroser et récolter carottes, salade, poivron, etc. Chaque légume a son effet ;
  - le **coin litière** (c'est là qu'on pousse les crottes pour garder Health) ;
  - une **mezzanine avec rampe** ;
  - des **tunnels** ;
  - des **emplacements de déco**, pour placer les trésors trouvés et les objets débloqués.

  Les cochons compagnons débloqués vivent ici, ainsi que le **tableau de quêtes** (des post-it). Une **petite porte/portillon**
  mène à la Bedroom : on l'ouvre d'un coup de nez.
  C'est aussi l'écran d'accueil de chaque session (« Continue » → on se réveille dans la PigRoom).
- **Bedroom** : la chambre de l'enfant, qui fait le lien entre la PigRoom et le reste de la maison. On y trouve le lit, des jouets
  et le coffre à jouets. Sous le lit : une zone sombre pleine de trésors perdus.
- **Kitchen** : la **fontaine à eau**, le frigo (événements de nourriture), la gamelle de Minou (interdite !), des miettes le soir.
  Le lieu du **raid de nuit**. Pipou la souris vit derrière la plinthe.
- **LivingRoom** : le canapé, la TV, le tapis de jeu, les tunnels, la station de **Robby l'aspirateur**, l'aquarium de Bulle.
  C'est la pièce des mini-jeux.
- Plus tard : **Bathroom** et **Garden** (herbe fraîche, papillons, danger oiseaux), débloqués en montant de niveau.

## Les personnages
**Les humains (en fond)** : ce sont des PNJ simples, qui traversent les pièces à certaines heures, avec peu d'animations
(marche + une pose). Ils ne donnent pas de quêtes.
- **L'enfant** : parfois un câlin surprise le soir (bonus Happiness).
- **Le parent** : il jette un légume le matin si tu fais Weeek au bon moment, et fait tomber des miettes au dîner.

**La communauté d'animaux (au cœur du jeu)** : chacun a une **jauge d'amitié**, des quêtes et des cadeaux.
- **Pipou la souris** (Kitchen) : le guide du tutoriel. Elle connaît les passages secrets et les trésors cachés.
- **Minou le chat** : d'abord **rival** (il bloque des passages, vole des légumes, tu le fais fuir au coup de nez).
  Au fil des quêtes, il devient **ami** et t'ouvre des zones en hauteur.
- **Robby l'aspirateur robot** : cassé au début. Une quête le **répare**, puis il nettoie une pièce par jour.
- **Cui-Cui l'oiseau** (à la fenêtre) : donne la météo du lendemain et des graines rares pour le potager.
- **Bulle le poisson rouge** (LivingRoom) : un sage un peu lunaire, qui donne les énigmes et les quêtes « trésor ».
- **Les autres cochons** (débloqués) : ils vivent dans la PigRoom, te suivent, jouent avec toi et donnent un bonus de Happiness quand ils sont ensemble.

## Mini-jeux (pour la Happiness)
1. **Tunnel Run** : courir dans des tunnels en carton, en évitant des obstacles (swipe).
2. **Cache-carotte** : trouver les légumes cachés par Pipou avant la fin du chrono.
3. **Nose Foot** : pousser une balle dans un but à coups de nez.
4. **Weeek Rhythm** : taper en rythme sur la musique pour faire chanter le cochon.

## Quêtes et XP
- **Quêtes du jour** (3 par jour, tirées au hasard, affichées sur le tableau de la PigRoom ou données par les animaux) :
  - « Récolte 3 carottes » ;
  - « Ramasse 5 crottes » ;
  - « Apporte une graine à Cui-Cui » ;
  - « Trouve 2 trésors avec le sonar » ;
  - « Visite toutes les pièces ».
- **Quêtes histoire** (tutoriel puis progression) :
  - « Premier jour dans la PigRoom » (tutoriel avec Pipou) ;
  - « Le trésor sous le lit » ;
  - « Réparer Robby » ;
  - « Le grand raid du frigo » ;
  - « Minou, mon ami ? » ;
  - « Un nouveau cochon ».
- **Collection** : des trésors cachés dans la maison (chaussette, bouchon, bille, clé…), à ramener à la PigRoom pour la décorer.
- **Récompenses de niveau** :
  - **Skins/races** : American, Abyssin, Péruvien, Teddy, Skinny.
  - **Accessoires** : chapeau, bandana, nœud.
  - **Déco de la PigRoom**.
  - **Nouveaux cochons** : niveaux 5, 10 et 15.
  - **Nouvelles pièces**.

## Temps et hors-ligne
- 1 journée de jeu ≈ 12–15 min réelles (adapté aux sessions mobiles).
- **Hors-ligne (proposition)** : les jauges baissent doucement quand l'app est fermée, **plafonné à −50 %**, pour qu'on ait
  envie de revenir voir son cochon sans être puni.

## Pourquoi ça marche en portrait
- Un seul pouce suffit : taper pour se déplacer, boutons Weeek et Coup de nez en bas de l'écran.
- Pièces verticales, avec une caméra qui défile doucement.
- Idéal pour des sessions courtes, un peu comme un Tamagotchi moderne.

## Répartition des rôles (version hybride retenue)
Rôles autrefois tenus par les humains, et ce qui les assure désormais (les humains ne font plus que des bonus ponctuels) :

| Rôle | Assuré par |
|---|---|
| Apporter à manger | **Distributeur automatique** dans la PigRoom : il se déclenche à heures fixes, et le cochon le lance plus tôt avec un Weeek. Un **potager** dans la PigRoom : planter, arroser et récolter les légumes, pour une petite boucle de ferme. Des **raids en cuisine** la nuit. |
| Remplir l'eau | **Fontaine à eau** dans la cuisine. Le distributeur d'eau de la PigRoom se recharge une fois par jour. |
| Faire le ménage | Le cochon pousse ses crottes à la litière. **Robby l'aspirateur robot** devient un allié : on le « répare » en quête, puis il nettoie une pièce par jour. |
| Câlins / jeux | Une **communauté d'animaux** : Pipou la souris (guide du tutoriel), Minou le chat (qui passe de rival à ami), un oiseau à la fenêtre, un poisson rouge… Il y a une **jauge d'amitié** par animal, les jouets et les mini-jeux. |
| Donner des quêtes | Les animaux PNJ, ainsi qu'un **tableau de quêtes** dans la PigRoom (des post-it du frigo). |
| Ouvrir le portillon | Le coup de nez. Les nouvelles pièces se débloquent par la progression (clé, chatière, trou de souris). |

- **Coût de production** : les humains se limitent à 2 PNJ de fond (marche + une pose). L'effort d'animation va aux animaux.

## Direction artistique — palette UI « Cosy »
Base : [colorhunt faf3f0-d4e2d4-ffcacc-dbc4f0](https://colorhunt.co/palette/faf3f0d4e2d4ffcaccdbc4f0).
Ces 4 pastels sont très doux, mais trop clairs pour du texte ou des contours lisibles en pixel art. J'ajoute donc
une **encre** foncée, une teinte **foncée par couleur** (contours, état appuyé, remplissage des jauges) et deux couleurs
manquantes (**bleu** pour l'eau, **pêche** pour la nourriture).

| Rôle | Clair (fond) | Foncé (remplissage / contour) | Usage |
|---|---|---|---|
| Crème (base) | `#FAF3F0` | `#E6D5CC` | fond des panneaux, écran de chargement, splash Android |
| Sauge | `#D4E2D4` | `#8FB08F` | jauge **Health**, boutons « OK / Continuer » |
| Rose | `#FFCACC` | `#E8868B` | jauge **Happiness**, bouton **Weeek**, cœurs |
| Lavande | `#DBC4F0` | `#A58BC9` | barre d'**XP**, quêtes, teinte de nuit |
| + Bleu | `#C9E1F2` | `#7FAED6` | jauge **Water** |
| + Pêche | `#FFE3C2` | `#F2A65A` | jauge **Food** |
| + Encre | `#4B3B47` | — | texte, contours 1 px, icônes |
| + Ombre | `#9C8A94` | — | ombres portées, éléments désactivés |

Règles d'usage :
- Tout texte en encre `#4B3B47` sur fond clair (contraste suffisant).
- Jamais de texte blanc sur un pastel.
- Panneaux : fond crème, bordure encre 1 px, coins de 2 px, ombre de 1 px en `#9C8A94` (ça donne un style pixel « carte »).
- Jauge : fond clair, remplissage foncé, contour encre. Elle clignote sous 20 % en alternant avec le rose foncé.
- Nuit : `CanvasModulate` teinté lavande foncé (`#6B5A8E`), au lieu d'un gris/noir. La nuit reste cosy et non inquiétante.
- Les sprites du jeu (cochon, décors) gardent leur palette GBC. Leurs couleurs claires sont harmonisées avec ces pastels (ex. la joue rose du cochon = `#FFCACC`).
- Implémentation : un `Theme` Godot (`assets/ui/cosy_theme.tres`) avec des `StyleBoxFlat`, et un script
  `autoload/palette.gd` (constantes `Color`) pour que le code utilise les mêmes couleurs.
  Mettre aussi `boot_splash/bg_color` et le splash Android sur `#FAF3F0`.

## Points à valider
- La durée d'une journée (12–15 min ?).
- La décroissance des jauges hors-ligne.
- Le casting des animaux (Pipou, Minou, Robby, Cui-Cui, Bulle) : à garder, à renommer ou à réduire pour la v1 ?
- Les mini-jeux à prioriser pour la v1 (proposition : **Nose Foot** et **Cache-carotte**, les plus simples).
- Plusieurs cochons ensemble (des compagnons qui te suivent) plutôt qu'un seul par partie.

