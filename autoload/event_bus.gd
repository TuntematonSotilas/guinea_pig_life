extends Node
## Global signals, so systems can talk without referencing each other.

@warning_ignore_start("unused_signal")
signal stat_changed(stat: StringName, value: float)
signal stat_depleted(stat: StringName)
signal pig_action(action: StringName)
signal quest_progressed(quest_id: StringName, progress: int)
signal quest_completed(quest_id: StringName)
signal xp_gained(amount: int)
signal leveled_up(level: int)
signal room_changed(room: StringName)
signal locale_changed(locale: String)
@warning_ignore_restore("unused_signal")
