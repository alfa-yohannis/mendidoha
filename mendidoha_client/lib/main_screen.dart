import 'package:flutter/material.dart';
import 'package:mendidoha_client/supplier/supplier_list_screen.dart';
import 'package:mendidoha_client/sign_in/sign_in_screen.dart';

import 'good/good_list_screen.dart'; // Import the SignInScreen widget

class MainScreen extends StatefulWidget {
  @override
  State<MainScreen> createState() => _MainScreenState();
}

class _MainScreenState extends State<MainScreen> {
  int _selectedIndex = 0;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Main Screen'),
      ),
      drawer: Drawer(
        child: ListView(
          padding: EdgeInsets.zero,
          children: <Widget>[
            DrawerHeader(
              decoration: BoxDecoration(
                color: Colors.blue,
              ),
              child: Text(
                'Main Menu',
                style: TextStyle(color: Colors.white, fontSize: 24),
              ),
            ),
            ListTile(
              leading: Icon(Icons.home),
              title: Text('Home'),
              onTap: () {
                _selectDrawerItem(0);
              },
            ),
            ExpansionTile(
              leading: Icon(Icons.settings),
              title: Text('Settings'),
              children: <Widget>[
                ListTile(
                  leading: Icon(Icons.person),
                  title: Text('Users'),
                  onTap: () {
                    _selectDrawerItem(1);
                  },
                ),
              ],
            ),
            ExpansionTile(
              leading: Icon(Icons.storage),
              title: Text('Master Data'),
              children: <Widget>[
                ListTile(
                  title: Text('Customer'),
                  onTap: () {
                    _selectDrawerItem(2);
                  },
                ),
                ListTile(
                  title: Text('Supplier'),
                  onTap: () {
                    _selectDrawerItem(3);
                  },
                ),
                ListTile(
                  title: Text('Goods'),
                  onTap: () {
                    _selectDrawerItem(4);
                  },
                ),
                ListTile(
                  title: Text('Services'),
                  onTap: () {
                    _selectDrawerItem(5);
                  },
                ),
              ],
            ),
            ListTile(
              leading: Icon(Icons.shopping_cart),
              title: Text('Sales Order'),
              onTap: () {
                _selectDrawerItem(6);
              },
            ),
            ListTile(
              leading: Icon(Icons.shopping_basket),
              title: Text('Purchase Order'),
              onTap: () {
                _selectDrawerItem(7);
              },
            ),
            ListTile(
              leading: Icon(Icons.exit_to_app),
              title: Text('Sign Out'),
              onTap: () {
                Navigator.pushReplacement(
                  context,
                  MaterialPageRoute(builder: (context) => SignInScreen()),
                );
              },
            ),
          ],
        ),
      ),
      body: Center(
        child: _buildSelectedScreen(_selectedIndex),
      ),
    );
  }

  void _selectDrawerItem(int index) {
    setState(() {
      _selectedIndex = index;
    });
    Navigator.pop(context); // Close the drawer
  }

  Widget _buildSelectedScreen(int selectedIndex) {
    switch (selectedIndex) {
      case 0:
        return _buildHomeScreen();
      case 1:
        return _buildUsersScreen();
      case 2:
        return _buildCustomerScreen();
      case 3:
        return SupplierListScreen();
      case 4:
        return GoodsListScreen();
      case 5:
        return _buildServicesScreen();
      case 6:
        return _buildSalesOrderScreen();
      case 7:
        return _buildPurchaseOrderScreen();
      default:
        return Container(); // Placeholder, add more cases as needed
    }
  }

  Widget _buildHomeScreen() {
    return Center(
      child: Text('Home Screen Content'),
    );
  }

  Widget _buildUsersScreen() {
    return Center(
      child: Text('Users Screen Content'),
    );
  }

  Widget _buildCustomerScreen() {
    return Center(
      child: Text('Customer Screen Content'),
    );
  }

  Widget _buildServicesScreen() {
    return Center(
      child: Text('Services Screen Content'),
    );
  }

  Widget _buildSalesOrderScreen() {
    return Center(
      child: Text('Sales Order Screen Content'),
    );
  }

  Widget _buildPurchaseOrderScreen() {
    return Center(
      child: Text('Purchase Order Screen Content'),
    );
  }
}
