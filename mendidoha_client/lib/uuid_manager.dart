import 'package:shared_preferences/shared_preferences.dart';
import 'package:uuid/uuid.dart';

class UuidManager {
  static const String uuidKey = 'device_id';

  static Future<String> getOrCreateUuid() async {
    final prefs = await SharedPreferences.getInstance();
    String? uuid = prefs.getString(uuidKey);

    if (uuid == null) {
      uuid = Uuid().v4(); // Generate a new UUID
      await prefs.setString(uuidKey, uuid);
    }

    return uuid;
  }
}
