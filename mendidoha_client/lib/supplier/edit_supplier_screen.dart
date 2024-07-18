import 'package:flutter/material.dart';

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

  @override
  void initState() {
    super.initState();
    _nameController = TextEditingController(text: widget.supplier['name']);
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
              controller: _nameController,
              decoration: InputDecoration(
                labelText: 'Supplier Name',
              ),
            ),
            SizedBox(height: 16.0),
            ElevatedButton(
              onPressed: () {
                String newName = _nameController.text.trim();
                Map<String, dynamic> updatedSupplier = {
                  ...widget.supplier,
                  'name': newName,
                };
                widget.onSupplierUpdated(updatedSupplier);
                Navigator.pop(context);
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
    super.dispose();
  }
}
