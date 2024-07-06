import 'package:flutter/material.dart';
import 'sign_in_screen.dart'; // Import the LoginScreen widget

void main() => runApp(MendidohaClient());

class MendidohaClient extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Mendidoha Client',
      theme: ThemeData(
        primarySwatch: Colors.blue,
      ),
      home: SignInScreen(), // Initial screen
    );
  }
}
