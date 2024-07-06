import 'package:flutter/material.dart';
import 'sign_in_screen.dart'; // Import the SignInScreen widget

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
                'Drawer Header',
                style: TextStyle(color: Colors.white, fontSize: 24),
              ),
            ),
            ListTile(
              leading: Icon(Icons.home),
              title: Text('Home'),
              onTap: () {
                // Update UI based on the selected item
                setState(() {
                  _selectedIndex = 0;
                });
                // Close the drawer
                Navigator.pop(context);
              },
            ),
            ListTile(
              leading: Icon(Icons.settings),
              title: Text('Settings'),
              onTap: () {
                // Update UI based on the selected item
                setState(() {
                  _selectedIndex = 1;
                });
                // Close the drawer
                Navigator.pop(context);
              },
            ),
            ListTile(
              leading: Icon(Icons.exit_to_app),
              title: Text('Sign Out'),
              onTap: () {
                // Navigate back to the sign-in screen
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

  Widget _buildSelectedScreen(int selectedIndex) {
    switch (selectedIndex) {
      case 0:
        return _buildHomeScreen();
      case 1:
        return _buildSettingsScreen();
      default:
        return Container(); // Placeholder, add more cases as needed
    }
  }

  Widget _buildHomeScreen() {
    return Center(
      child: Text('Home Screen Content'),
    );
  }

  Widget _buildSettingsScreen() {
    return Center(
      child: Text('Settings Screen Content'),
    );
  }
}
