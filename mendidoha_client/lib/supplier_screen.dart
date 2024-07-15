import 'package:flutter/material.dart';
import 'supplier_list_screen.dart';

class SupplierScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Supplier List'),
      ),
      body: SupplierList(),
    );
  }
}
