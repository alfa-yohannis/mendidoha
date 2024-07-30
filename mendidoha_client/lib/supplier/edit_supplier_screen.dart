import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:mendidoha_client/config.dart';

class EditSupplierScreen extends StatefulWidget {
  final Map<String, dynamic> supplier;
  final Function(Map<String, dynamic>) onSupplierUpdated;

  const EditSupplierScreen({
    super.key,
    required this.supplier,
    required this.onSupplierUpdated,
  });

  @override
  _EditSupplierScreenState createState() => _EditSupplierScreenState();
}

class _EditSupplierScreenState extends State<EditSupplierScreen> {
  late TextEditingController _nameController;
  late TextEditingController _codeController;

  @override
  void initState() {
    super.initState();
    _nameController = TextEditingController(text: widget.supplier['name']);
    _codeController = TextEditingController(text: widget.supplier['code']);
  }

  Future<void> _updateSupplier(String name, String code) async {
    SharedPreferences prefs = await SharedPreferences.getInstance();
    String? sessionId = prefs.getString('session_id');
    String? deviceId = prefs.getString('device_id');
    String? username = prefs.getString('username');

    if (sessionId == null || deviceId == null || username == null) {
      print('Session ID, Device ID, or Username not found');
      return;
    }

    final Map<String, dynamic> requestData = {
      'id': widget.supplier['id'],
      'name': name,
      'code': code,
      'session_id': sessionId,
      'device_id': deviceId,
      'username': username,
    };

    final response = await http.post(
      Uri.parse('${AppConfig.apiUrl}/suppliers/update'),
      headers: <String, String>{
        'Content-Type': 'application/json; charset=UTF-8',
      },
      body: jsonEncode(requestData),
    );

    if (response.statusCode == 200) {
      final Map<String, dynamic> responseData = jsonDecode(response.body);
      String updatedName = responseData['name'] ?? '';
      String updatedCode = responseData['code'] ?? '';

      widget.onSupplierUpdated({
        'id': widget.supplier['id'],
        'name': updatedName,
        'code': updatedCode,
      });
      Navigator.pop(context);
    } else {
      print('Failed to update supplier');
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Edit Supplier'),
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
                fillColor: Colors.grey[200], // Background color to indicate it's uneditable
                labelText: 'Supplier Code (uneditable for the moment)',
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
                String name = _nameController.text.trim();
                String code = _codeController.text.trim();
                if (name.isNotEmpty) {
                  _updateSupplier(name, code);
                }
              },
              child: Text('Save'),
            ),
          ],
        ),
      ),
    );
  }

  @override
  void dispose() {
    _nameController.dispose();
    _codeController.dispose();
    super.dispose();
  }
}
