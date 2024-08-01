import 'dart:convert';
import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'package:mendidoha_client/config.dart';
import 'package:shared_preferences/shared_preferences.dart';
import 'package:mendidoha_client/good/edit_good_screen.dart';
import 'add_good_screen.dart'; // Import AddGoodsScreen

class GoodsListScreen extends StatefulWidget {
  @override
  _GoodsListScreenState createState() => _GoodsListScreenState();
}

class _GoodsListScreenState extends State<GoodsListScreen> {
  List<Map<String, dynamic>> _goods = [];

  TextEditingController _filterController = TextEditingController();
  String _searchTerm = '';
  int _sortColumnIndex = 0;
  bool _sortAscending = true;

  @override
  void initState() {
    super.initState();
    _fetchGoods('');
  }

  Future<void> _fetchGoods(String searchTerm) async {
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
      Uri.parse('${AppConfig.apiUrl}/goods'),
      headers: <String, String>{
        'Content-Type': 'application/json; charset=UTF-8',
      },
      body: jsonEncode(requestData),
    );

    if (response.statusCode == 200) {
      final List<dynamic> data = jsonDecode(response.body);
      setState(() {
        _goods = data
            .map((item) => {
                  'code': item['code'],
                  'name': item['name'],
                  'price': item['price'],
                  'quantity': item['quantity'],
                  'unit': item['unit'],
                  'currency': item['currency'],
                })
            .toList();
      });
    } else {
      print('Failed to fetch goods');
    }
  }

  Future<void> _deleteGoods(int index) async {
    SharedPreferences prefs = await SharedPreferences.getInstance();
    String? sessionId = prefs.getString('session_id');
    String? deviceId = prefs.getString('device_id');
    String? username = prefs.getString('username');
    String goodsCode = _goods[index]['code'];

    if (sessionId == null || deviceId == null || username == null) {
      print('Session ID, Device ID, or Username not found');
      return;
    }

    final Map<String, dynamic> requestData = {
      'session_id': sessionId,
      'device_id': deviceId,
      'username': username,
      'code': goodsCode,
    };

    final response = await http.post(
      Uri.parse('${AppConfig.apiUrl}/goods/delete'),
      headers: <String, String>{
        'Content-Type': 'application/json; charset=UTF-8',
      },
      body: jsonEncode(requestData),
    );

    if (response.statusCode == 200) {
      setState(() {
        _goods.removeAt(index);
      });
    } else {
      print('Failed to delete goods');
    }
  }

  void _showDeleteConfirmationDialog(int index) {
    showDialog(
      context: context,
      builder: (BuildContext context) {
        return AlertDialog(
          title: Text('Confirm Delete'),
          content: Text('Are you sure you want to delete this goods item?'),
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
                _deleteGoods(index);
                Navigator.of(context).pop();
              },
            ),
          ],
        );
      },
    );
  }

  List<Map<String, dynamic>> _filteredGoods() {
    var filteredList = _goods;

    int columnCount = _goods.isNotEmpty ? _goods[0].length : 0;
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

  void _editGoods(int index) async {
    final result = await Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => EditGoodsScreen(
          goods: _goods[index],
          onGoodsUpdated: (Map<String, dynamic> updatedGoods) {
            setState(() {
              _goods[index] = updatedGoods;
            });
          },
        ),
      ),
    );
    if (result != null && result is Map<String, dynamic>) {
      // Handle result if needed
    }
  }

  void _navigateToAddGoods(BuildContext context) async {
    final result = await Navigator.push(
      context,
      MaterialPageRoute(
        builder: (context) => AddGoodsScreen(
          onGoodsAdded: (Map<String, dynamic> newGoods) {
            setState(() {
              _goods.add({
                'code': newGoods['code'],
                'name': newGoods['name'],
                'price': newGoods['price'],
                'quantity': newGoods['quantity'],
                'unit': newGoods['unit'],
                'currency': newGoods['currency'],
              });
            });
            return newGoods['code'];
          },
        ),
      ),
    );
    if (result != null && result is Map<String, dynamic>) {
      // Handle result if needed
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Goods List'),
      ),
      body: Column(
        children: [
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: TextField(
              controller: _filterController,
              decoration: InputDecoration(
                labelText: 'Filter by Goods Name or Code',
                suffixIcon: IconButton(
                  icon: Icon(Icons.search),
                  onPressed: () {
                    setState(() {
                      _searchTerm = _filterController.text;
                    });
                    _fetchGoods(_searchTerm);
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
                    label: Text('Price'),
                    onSort: (columnIndex, ascending) {
                      _onSort(columnIndex, ascending);
                    },
                  ),
                  DataColumn(
                    label: Text('Quantity'),
                    onSort: (columnIndex, ascending) {
                      _onSort(columnIndex, ascending);
                    },
                  ),
                  DataColumn(
                    label: Text('Unit'),
                    onSort: (columnIndex, ascending) {
                      _onSort(columnIndex, ascending);
                    },
                  ),
                  DataColumn(
                    label: Text('Currency'),
                    onSort: (columnIndex, ascending) {
                      _onSort(columnIndex, ascending);
                    },
                  ),
                  DataColumn(
                    label: Text('Actions'),
                  ),
                ],
                rows: _filteredGoods().map((goods) {
                  int index = _goods.indexOf(goods);
                  return DataRow(cells: [
                    DataCell(Text(goods['no'].toString())),
                    DataCell(Text(goods['code'])),
                    DataCell(Text(goods['name'])),
                    DataCell(Text(goods['price'].toString())),
                    DataCell(Text(goods['quantity'].toString())),
                    DataCell(Text(goods['unit'])),
                    DataCell(Text(goods['currency'])),
                    DataCell(Row(
                      children: [
                        IconButton(
                          icon: Icon(Icons.edit),
                          onPressed: () {
                            _editGoods(index);
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
          _navigateToAddGoods(context);
        },
                tooltip: 'New Goods',
        child: Icon(Icons.add),
      ),
    );
  }
}

