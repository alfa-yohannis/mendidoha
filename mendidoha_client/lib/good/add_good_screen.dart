import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:mendidoha_client/config.dart';

class AddGoodsScreen extends StatefulWidget {
  final Function(Map<String, dynamic>) onGoodsAdded;

  const AddGoodsScreen({super.key, required this.onGoodsAdded});

  @override
  _AddGoodsScreenState createState() => _AddGoodsScreenState();
}

class _AddGoodsScreenState extends State<AddGoodsScreen> {
  final TextEditingController _nameController = TextEditingController();
  final TextEditingController _codeController = TextEditingController();
  final TextEditingController _priceController = TextEditingController();
  final TextEditingController _quantityController = TextEditingController();
  final TextEditingController _unitController = TextEditingController();
  final TextEditingController _currencyController = TextEditingController();

  Future<void> _addGoods() async {
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
      'code': _codeController.text.trim(),
      'price': double.parse(_priceController.text.trim()),
      'quantity': double.parse(_quantityController.text.trim()),
      'unit': _unitController.text.trim(),
      'currency': _currencyController.text.trim(),
      'session_id': sessionId,
      'device_id': deviceId,
      'username': username,
    };

    final response = await http.post(
      Uri.parse('${AppConfig.apiUrl}/goods/add'),
      headers: <String, String>{
        'Content-Type': 'application/json; charset=UTF-8',
      },
      body: jsonEncode(requestData),
    );

    if (response.statusCode == 200) {
      final Map<String, dynamic> responseData = jsonDecode(response.body);
      String name = responseData['name'] ?? '';
      String code = responseData['code'] ?? '';

      widget.onGoodsAdded({
        'name': name,
        'code': code,
        'price': requestData['price'],
        'quantity': requestData['quantity'],
        'unit': requestData['unit'],
        'currency': requestData['currency'],
      });
      Navigator.pop(context);
    } else {
      print('Failed to add goods');
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Add Goods'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            TextField(
              controller: _codeController,
              decoration: InputDecoration(
                labelText: 'Goods Code',
              ),
            ),
            SizedBox(height: 16.0),
            TextField(
              controller: _nameController,
              decoration: InputDecoration(
                labelText: 'Goods Name',
              ),
            ),
            SizedBox(height: 16.0),
            TextField(
              controller: _priceController,
              keyboardType: TextInputType.numberWithOptions(decimal: true),
              decoration: InputDecoration(
                labelText: 'Price',
              ),
            ),
            SizedBox(height: 16.0),
            TextField(
              controller: _quantityController,
              keyboardType: TextInputType.numberWithOptions(decimal: true),
              decoration: InputDecoration(
                labelText: 'Quantity',
              ),
            ),
            SizedBox(height: 16.0),
            TextField(
              controller: _unitController,
              decoration: InputDecoration(
                labelText: 'Unit',
              ),
            ),
            SizedBox(height: 16.0),
            TextField(
              controller: _currencyController,
              decoration: InputDecoration(
                labelText: 'Currency',
              ),
            ),
            SizedBox(height: 16.0),
            ElevatedButton(
              onPressed: () {
                if (_nameController.text.trim().isNotEmpty &&
                    _codeController.text.trim().isNotEmpty &&
                    _priceController.text.trim().isNotEmpty &&
                    _quantityController.text.trim().isNotEmpty &&
                    _currencyController.text.trim().isNotEmpty) {
                  _addGoods();
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
