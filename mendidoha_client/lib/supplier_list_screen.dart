import 'package:flutter/material.dart';

class SupplierList extends StatefulWidget {
  @override
  _SupplierListState createState() => _SupplierListState();
}

class _SupplierListState extends State<SupplierList> {
  List<Map<String, dynamic>> _suppliers = [
    {
      'code': '1000000001',
      'name': 'Supplier A Inc.',
      'created': '2024-07-16',
      'updated': '2024-07-16',
      'created_by': 'User A',
      'updated_by': 'User B',
    },
    {
      'code': '1000000002',
      'name': 'Supplier B Enterprises',
      'created': '2024-07-16',
      'updated': '2024-07-16',
      'created_by': 'User C',
      'updated_by': 'User D',
    },
    {
      'code': '1000000003',
      'name': 'Supplier C Ltd.',
      'created': '2024-07-16',
      'updated': '2024-07-16',
      'created_by': 'User E',
      'updated_by': 'User F',
    },
    // Add more suppliers as needed
  ];

  TextEditingController _filterController = TextEditingController();
  String _searchTerm = '';
  int _sortColumnIndex = 0;
  bool _sortAscending = true;

  @override
  void initState() {
    super.initState();
    _injectLineNumbers();
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
          .where((supplier) =>
              supplier['name'].toLowerCase().contains(_searchTerm.toLowerCase()))
          .toList();
    }

    // Ensure _sortColumnIndex is within the range of columns
    int columnCount = _suppliers.isNotEmpty ? _suppliers[0].length : 0;
    if (_sortColumnIndex != null && _sortColumnIndex <= columnCount) {
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

  @override
  Widget build(BuildContext context) {
    return Column(
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
                  label: Text('Created'),
                  onSort: (columnIndex, ascending) {
                    _onSort(columnIndex, ascending);
                  },
                ),
                DataColumn(
                  label: Text('Updated'),
                  onSort: (columnIndex, ascending) {
                    _onSort(columnIndex, ascending);
                  },
                ),
                DataColumn(
                  label: Text('Created By'),
                  onSort: (columnIndex, ascending) {
                    _onSort(columnIndex, ascending);
                  },
                ),
                DataColumn(
                  label: Text('Updated By'),
                  onSort: (columnIndex, ascending) {
                    _onSort(columnIndex, ascending);
                  },
                ),
              ],
              rows: _filteredSuppliers().map((supplier) {
                return DataRow(cells: [
                  DataCell(Text(supplier['no'].toString())),
                  DataCell(Text(supplier['code'])),
                  DataCell(Text(supplier['name'])),
                  DataCell(Text(supplier['created'])),
                  DataCell(Text(supplier['updated'])),
                  DataCell(Text(supplier['created_by'])),
                  DataCell(Text(supplier['updated_by'])),
                ]);
              }).toList(),
            ),
          ),
        ),
      ],
    );
  }
}
