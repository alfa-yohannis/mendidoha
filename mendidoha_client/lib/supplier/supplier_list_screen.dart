import 'dart:convert';
import 'dart:math';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'package:mendidoha_client/config.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:mendidoha_client/supplier/edit_supplier_screen.dart';
import 'add_supplier_screen.dart'; // Import AddSupplierScreen

class SupplierListScreen extends StatefulWidget {
  @override
  _SupplierListScreenState createState() => _SupplierListScreenState();
}

class _SupplierListScreenState extends State<SupplierListScreen> {
  List<Map<String, dynamic>> _suppliers = [];

  TextEditingController _filterController = TextEditingController();
  String _searchTerm = '';
  int _sortColumnIndex = 0;
  bool _sortAscending = true;

  @override
  void initState() {
    super.initState();
    _fetchSuppliers('');
  }

  Future<void> _fetchSuppliers(String searchTerm) async {
    SharedPreferences prefs = await SharedPreferences.getInstance();
    String? sessionId = prefs.getString('session_id');
    String? deviceId = prefs.getString('device_id');
    String? username = prefs.getString('username');

    if (sessionId == null || deviceId == null || username == null) {
      print('Session ID, Device ID, or Username not found');
      return;
    }

    final Map<String, dynamic> requestData = {
      'search_string': searchTerm,
      'session_id': sessionId,
      'device_id': deviceId,
      'username': username,
    };

    final response = await http.post(
      Uri.parse('${AppConfig.apiUrl}/suppliers'),
      headers: <String, String>{
        'Content-Type': 'application/json; charset=UTF-8',
      },
      body: jsonEncode(requestData),
    );

    if (response.statusCode == 200) {
      final List<dynamic> data = jsonDecode(response.body);
      setState(() {
        _suppliers = data
            .map((item) => {
                  'code': item['code'],
                  'name': item['name'],
                })
            .toList();
      });
    } else {
      print('Failed to fetch suppliers');
    }
  }

  Future<void> _deleteSupplier(int index) async {
    SharedPreferences prefs = await SharedPreferences.getInstance();
    String? sessionId = prefs.getString('session_id');
    String? deviceId = prefs.getString('device_id');
    String? username = prefs.getString('username');
    String supplierCode = _suppliers[index]['code'];

    if (sessionId == null || deviceId == null || username == null) {
      print('Session ID, Device ID, or Username not found');
      return;
    }

    final Map<String, dynamic> requestData = {
      'session_id': sessionId,
      'device_id': deviceId,
      'username': username,
      'code': supplierCode,
    };

    final response = await http.post(
      Uri.parse('${AppConfig.apiUrl}/suppliers/delete'),
      headers: <String, String>{
        'Content-Type': 'application/json; charset=UTF-8',
      },
      body: jsonEncode(requestData),
    );

    if (response.statusCode == 200) {
      setState(() {
        _suppliers.removeAt(index);
      });
    } else {
      print('Failed to delete supplier');
    }
  }

  void _showDeleteConfirmationDialog(int index) {
    showDialog(
      context: context,
      builder: (BuildContext context) {
        return AlertDialog(
          title: Text('Confirm Delete'),
          content: Text('Are you sure you want to delete this supplier?'),
          actions: <Widget>[
            TextButton(
              child: Text('Cancel'),
              onPressed: () {
                Navigator.of(context).pop();
              },
            ),
            TextButton(
              child: Text('Delete'),
              onPressed: () {
                _deleteSupplier(index);
                Navigator.of(context).pop();
              },
            ),
          ],
        );
      },
    );
  }

  List<Map<String, dynamic>> _filteredSuppliers() {
    var filteredList = _suppliers;

    int columnCount = _suppliers.isNotEmpty ? _suppliers[0].length : 0;
    if (_sortColumnIndex <= columnCount) {
      filteredList.sort((a, b) {
        dynamic aValue = a.values.toList()[_sortColumnIndex];
        dynamic bValue = b.values.toList()[_sortColumnIndex];
        if (_sortAscending) {
          return aValue.toString().compareTo(bValue.toString());
        } else {
          return bValue.toString().compareTo(aValue.toString());
        }
      });
    }

    for (int i = 0; i < filteredList.length; i++) {
      filteredList[i]['no'] = i + 1;
    }

    return filteredList;
  }

  void _onSort(int columnIndex, bool ascending) {
    setState(() {
      _sortColumnIndex = columnIndex;
      _sortAscending = ascending;
    });
  }

  void _editSupplier(int index) async {
    final result = await Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => EditSupplierScreen(
          supplier: _suppliers[index],
          onSupplierUpdated: (Map<String, dynamic> updatedSupplier) {
            setState(() {
              _suppliers[index] = updatedSupplier;
            });
          },
        ),
      ),
    );
    if (result != null) {
      // Handle any result if needed
    }
  }

  void _navigateToAddSupplier(BuildContext context) async {
    final result = await Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => AddSupplierScreen(
          onSupplierAdded: (Map<String, dynamic> newSupplier) {
            setState(() {
              _suppliers.add({
                'code': newSupplier['code'],
                'name': newSupplier['name'],
              });
            });
            return newSupplier['code'];
          },
        ),
      ),
    );
    if (result != null && result is Map<String, dynamic>) {
      // Handle result if needed
    }
  }

  String _generateSupplierCode() {
    Random rnd = Random();

    String code = (rnd.nextInt(9) + 1).toString();

    for (int i = 1; i < 10; i++) {
      code += rnd.nextInt(10).toString();
    }

    return code;
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Supplier List'),
      ),
      body: Column(
        children: [
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: TextField(
              controller: _filterController,
              decoration: InputDecoration(
                labelText: 'Filter by Supplier Name or Code',
                suffixIcon: IconButton(
                  icon: Icon(Icons.search),
                  onPressed: () {
                    setState(() {
                      _searchTerm = _filterController.text;
                    });
                    _fetchSuppliers(_searchTerm);
                  },
                ),
              ),
            ),
          ),
          Expanded(
            child: SingleChildScrollView(
              scrollDirection: Axis.vertical,
              child: DataTable(
                sortColumnIndex: _sortColumnIndex,
                sortAscending: _sortAscending,
                columns: [
                  DataColumn(
                    label: Text('No.'),
                    onSort: (columnIndex, ascending) {
                      _onSort(columnIndex, ascending);
                    },
                  ),
                  DataColumn(
                    label: Text('Code'),
                    onSort: (columnIndex, ascending) {
                      _onSort(columnIndex, ascending);
                    },
                  ),
                  DataColumn(
                    label: Text('Name'),
                    onSort: (columnIndex, ascending) {
                      _onSort(columnIndex, ascending);
                    },
                  ),
                  DataColumn(
                    label: Text('Actions'),
                  ),
                ],
                rows: _filteredSuppliers().map((supplier) {
                  int index = _suppliers.indexOf(supplier);
                  return DataRow(cells: [
                    DataCell(Text(supplier['no'].toString())),
                    DataCell(Text(supplier['code'])),
                    DataCell(Text(supplier['name'])),
                    DataCell(Row(
                      children: [
                        IconButton(
                          icon: Icon(Icons.edit),
                          onPressed: () {
                            _editSupplier(index);
                          },
                        ),
                        IconButton(
                          icon: Icon(Icons.delete),
                          onPressed: () {
                            _showDeleteConfirmationDialog(index);
                          },
                        ),
                      ],
                    )),
                  ]);
                }).toList(),
              ),
            ),
          ),
        ],
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: () {
          _navigateToAddSupplier(context);
        },
        tooltip: 'New Supplier',
        child: Icon(Icons.add),
      ),
    );
  }
}
