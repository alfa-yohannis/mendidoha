import 'dart:convert';
import 'dart:math';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
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
    _fetchSuppliers();
  }

  Future<void> _fetchSuppliers() async {
    SharedPreferences prefs = await SharedPreferences.getInstance();
    String? sessionId = prefs.getString('session_id');

    if (sessionId == null) {
      // Handle case where session_id is not found
      print('Session ID not found');
      return;
    }

    final response = await http.get(
      Uri.parse('http://0.0.0.0:8080/list_suppliers'),
      headers: {
        'Content-Type': 'application/json',
        'session_id': sessionId,
      },
    );

    if (response.statusCode == 200) {
      final List<dynamic> data = jsonDecode(response.body);
      setState(() {
        _suppliers = data.map((item) => {
          'code': item['code'],
          'name': item['name'],
          'no': _suppliers.length + 1, // Assuming 'no' is the line number
        }).toList();
      });
    } else {
      // Handle error
      print('Failed to fetch suppliers');
    }
  }

  void _injectLineNumbers() {
    for (int i = 0; i < _suppliers.length; i++) {
      _suppliers[i]['no'] = i + 1;
    }
  }

  List<Map<String, dynamic>> _filteredSuppliers() {
    var filteredList = _suppliers;
    if (_searchTerm.isNotEmpty) {
      filteredList = filteredList
          .where((supplier) => supplier['name']
              .toLowerCase()
              .contains(_searchTerm.toLowerCase()))
          .toList();
    }

    // Ensure _sortColumnIndex is within the range of columns
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

  void _deleteSupplier(int index) {
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
                setState(() {
                  _suppliers.removeAt(index);
                  _injectLineNumbers(); // Update line numbers after deletion
                });
                Navigator.of(context).pop();
              },
            ),
          ],
        );
      },
    );
  }

  void _navigateToAddSupplier(BuildContext context) async {
    final result = await Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => AddSupplierScreen(
          onSupplierAdded: (Map<String, dynamic> newSupplier) {
            String code =
                _generateSupplierCode(); // Generate code for new supplier
            setState(() {
              _suppliers.add({
                'code': code,
                'name': newSupplier['name'],
                'no': _suppliers.length + 1, // Assuming 'no' is the line number
              });
              _injectLineNumbers();
            });
            return code; // Return the generated code
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

    // Generate first digit (1-9)
    String code = (rnd.nextInt(9) + 1).toString();

    // Generate remaining 9 digits
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
                labelText: 'Filter by Supplier Name',
                suffixIcon: IconButton(
                  icon: Icon(Icons.search),
                  onPressed: () {
                    setState(() {
                      _searchTerm = _filterController.text;
                    });
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
                            _deleteSupplier(index);
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
