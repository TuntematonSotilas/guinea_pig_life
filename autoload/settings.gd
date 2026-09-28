extends Node
## Player preferences (language, volume). Persisted by SaveManager.

const SUPPORTED_LOCALES: PackedStringArray = ["en", "fr", "de", "es"]

var locale: String = ""
var music_volume: float = 0.8
var sfx_volume: float = 1.0
var vibration: bool = true


func _ready() -> void:
	# First launch: follow the phone language when we support it.
	var os_lang := OS.get_locale_language()
	set_locale(os_lang if os_lang in SUPPORTED_LOCALES else "en")


func set_locale(new_locale: String) -> void:
	if new_locale not in SUPPORTED_LOCALES or new_locale == locale:
		return
	locale = new_locale
	TranslationServer.set_locale(locale)
	EventBus.locale_changed.emit(locale)


func to_dict() -> Dictionary:
	return {
		"locale": locale,
		"music_volume": music_volume,
		"sfx_volume": sfx_volume,
		"vibration": vibration,
	}


func from_dict(data: Dictionary) -> void:
	music_volume = data.get("music_volume", music_volume)
	sfx_volume = data.get("sfx_volume", sfx_volume)
	vibration = data.get("vibration", vibration)
	set_locale(data.get("locale", locale))
