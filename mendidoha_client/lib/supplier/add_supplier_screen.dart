import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:mendidoha_client/config.dart';

class AddSupplierScreen extends StatefulWidget {
  final Function(Map<String, dynamic>) onSupplierAdded;

  const AddSupplierScreen({super.key, required this.onSupplierAdded});

  @override
  _AddSupplierScreenState createState() => _AddSupplierScreenState();
}

class _AddSupplierScreenState extends State<AddSupplierScreen> {
  final TextEditingController _nameController = TextEditingController();
  final TextEditingController _codeController = TextEditingController();

  Future<void> _addSupplier() async {
    SharedPreferences prefs = await SharedPreferences.getInstance();
    String? sessionId = prefs.getString('session_id');
    String? deviceId = prefs.getString('device_id');
    String? username = prefs.getString('username');

    if (sessionId == null || deviceId == null || username == null) {
      print('Session ID, Device ID, or Username not found');
      return;
    }

    final Map<String, dynamic> requestData = {
      'name': _nameController.text.trim(),
      'session_id': sessionId,
      'device_id': deviceId,
      'username': username,
      'code': '',
    };

    final response = await http.post(
      Uri.parse('${AppConfig.apiUrl}/suppliers/add'),
      headers: <String, String>{
        'Content-Type': 'application/json; charset=UTF-8',
      },
      body: jsonEncode(requestData),
    );

    if (response.statusCode == 200) {
      final Map<String, dynamic> responseData = jsonDecode(response.body);
      String name = responseData['name'] ?? '';
      String code = responseData['code'] ?? '';

      widget.onSupplierAdded({'name': name, 'code': code});
      Navigator.pop(context);
    } else {
      print('Failed to add supplier');
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Add Supplier'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            TextField(
              controller: _codeController,
              readOnly: true,
              style: TextStyle(color: Colors.black),
              decoration: InputDecoration(
                filled: true,
                fillColor: Colors
                    .grey[200], // Background color to indicate it's uneditable
                labelText:
                    'Supplier Code (will be generated later, uneditable for the moment)',
              ),
            ),
            SizedBox(height: 16.0),
            TextField(
              controller: _nameController,
              decoration: InputDecoration(
                labelText: 'Supplier Name',
              ),
            ),
            SizedBox(height: 16.0),
            ElevatedButton(
              onPressed: () {
                if (_nameController.text.trim().isNotEmpty) {
                  _addSupplier();
                }
              },
              child: Text('Save'),
            ),
          ],
        ),
      ),
    );
  }
}
