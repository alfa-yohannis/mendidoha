import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:mendidoha_client/config.dart';

class EditGoodsScreen extends StatefulWidget {
  final Map<String, dynamic> goods;
  final Function(Map<String, dynamic>) onGoodsUpdated;

  const EditGoodsScreen({
    super.key,
    required this.goods,
    required this.onGoodsUpdated,
  });

  @override
  _EditGoodsScreenState createState() => _EditGoodsScreenState();
}

class _EditGoodsScreenState extends State<EditGoodsScreen> {
  late TextEditingController _nameController;
  late TextEditingController _codeController;
  late TextEditingController _priceController;
  late TextEditingController _quantityController;
  late TextEditingController _unitController;
  late TextEditingController _currencyController;

  @override
  void initState() {
    super.initState();
    _nameController = TextEditingController(text: widget.goods['name']);
    _codeController = TextEditingController(text: widget.goods['code']);
    _priceController = TextEditingController(text: widget.goods['price'].toString());
    _quantityController = TextEditingController(text: widget.goods['quantity'].toString());
    _unitController = TextEditingController(text: widget.goods['unit']);
    _currencyController = TextEditingController(text: widget.goods['currency']);
  }

  Future<void> _updateGoods(Map<String, dynamic> updatedGoods) async {
    SharedPreferences prefs = await SharedPreferences.getInstance();
    String? sessionId = prefs.getString('session_id');
    String? deviceId = prefs.getString('device_id');
    String? username = prefs.getString('username');

    if (sessionId == null || deviceId == null || username == null) {
      print('Session ID, Device ID, or Username not found');
      return;
    }

    updatedGoods.addAll({
      'session_id': sessionId,
      'device_id': deviceId,
      'username': username,
    });

    final response = await http.post(
      Uri.parse('${AppConfig.apiUrl}/goods/update'),
      headers: <String, String>{
        'Content-Type': 'application/json; charset=UTF-8',
      },
      body: jsonEncode(updatedGoods),
    );

    if (response.statusCode == 200) {
      final Map<String, dynamic> responseData = jsonDecode(response.body);

      widget.onGoodsUpdated({
        'id': widget.goods['id'],
        'name': responseData['name'] ?? '',
        'code': responseData['code'] ?? '',
        'price': responseData['price'] ?? 0.0,
        'quantity': responseData['quantity'] ?? 0.0,
        'unit': responseData['unit'] ?? '',
        'currency': responseData['currency'] ?? '',
      });
      Navigator.pop(context);
    } else {
      print('Failed to update goods');
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Edit Goods'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: SingleChildScrollView(
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
                  labelText: 'Goods Code (uneditable for the moment)',
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
                decoration: InputDecoration(
                  labelText: 'Price',
                  suffixText: 'currency',
                ),
                keyboardType: TextInputType.number,
              ),
              SizedBox(height: 16.0),
              TextField(
                controller: _quantityController,
                decoration: InputDecoration(
                  labelText: 'Quantity',
                ),
                keyboardType: TextInputType.number,
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
                  Map<String, dynamic> updatedGoods = {
                    'id': widget.goods['id'],
                    'name': _nameController.text.trim(),
                    'code': _codeController.text.trim(),
                    'price': double.tryParse(_priceController.text.trim()) ?? 0.0,
                    'quantity': double.tryParse(_quantityController.text.trim()) ?? 0.0,
                    'unit': _unitController.text.trim(),
                    'currency': _currencyController.text.trim(),
                  };
                  if (updatedGoods['name'].isNotEmpty) {
                    _updateGoods(updatedGoods);
                  }
                },
                child: Text('Save'),
              ),
            ],
          ),
        ),
      ),
    );
  }

  @override
  void dispose() {
    _nameController.dispose();
    _codeController.dispose();
    _priceController.dispose();
    _quantityController.dispose();
    _unitController.dispose();
    _currencyController.dispose();
    super.dispose();
  }
}
