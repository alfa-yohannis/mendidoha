import 'package:shared_preferences/shared_preferences.dart';
import 'package:uuid/uuid.dart';

class UuidManager {
  static const String deviceIdKey = 'device_id';

  static Future<String> getOrCreateDeviceId() async {
    final prefs = await SharedPreferences.getInstance();
    String? uuid = prefs.getString(deviceIdKey);

    if (uuid == null) {
      uuid = Uuid().v4(); // Generate a new UUID
      await prefs.setString(deviceIdKey, uuid);
    }

    return uuid;
  }
}
