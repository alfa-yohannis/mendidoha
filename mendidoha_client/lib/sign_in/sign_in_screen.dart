import 'package:flutter/material.dart';
import 'dart:convert'; // for jsonEncode and jsonDecode
import 'package:http/http.dart' as http; // import http package
import '../main_screen.dart'; // Import the MainScreen widget
import '../sign_up/sign_up_screen.dart'; // Import the SignUpScreen widget
import '../reset_password/reset_password_screen.dart'; // Import the ResetPasswordScreen widget
import 'package:mendidoha_client/config.dart'; 

class SignInScreen extends StatelessWidget {
  final _formKey = GlobalKey<FormState>();
  final TextEditingController _usernameController = TextEditingController();
  final TextEditingController _passwordController = TextEditingController();
  final FocusNode _usernameFocusNode = FocusNode(); // Add FocusNode for username
  final FocusNode _passwordFocusNode = FocusNode();

  // Hardcoded credentials for validation
  final String validUsername = AppConfig.username;
  final String validPassword = AppConfig.password;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Sign In'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Form(
          key: _formKey,
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: <Widget>[
              // Add logo image
              Image.asset(
                'assets/images/logo.png',
                height: 128, // Adjust the height to fit your design
              ),
              SizedBox(height: 24.0), // Space between logo and the form
              TextFormField(
                controller: _usernameController,
                focusNode: _usernameFocusNode, // Attach FocusNode to username field
                decoration: InputDecoration(
                  labelText: 'Username (Email)',
                  border: OutlineInputBorder(),
                ),
                validator: (value) {
                  // if (value == null || value.isEmpty) {
                  //   return 'Please enter your username';
                  // } 
                  // else if (!RegExp(r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$').hasMatch(value)) {
                  //   return 'Please enter a valid email address';
                  // }
                  return null;
                },
                onFieldSubmitted: (_) {
                  _passwordFocusNode.requestFocus();
                },
                autofocus: true, // Set username field to be focused by default
              ),
              SizedBox(height: 16.0),
              TextFormField(
                controller: _passwordController,
                focusNode: _passwordFocusNode,
                obscureText: true,
                decoration: InputDecoration(
                  labelText: 'Password',
                  border: OutlineInputBorder(),
                ),
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter your password';
                  }
                  return null;
                },
                onFieldSubmitted: (_) {
                  // Trigger sign-in logic when password field submitted
                  _signIn(context);
                },
              ),
              SizedBox(height: 16.0),
              ElevatedButton(
                onPressed: () {
                  // Trigger sign-in logic when button pressed
                  _signIn(context);
                },
                child: Text('Sign In'),
              ),
              SizedBox(height: 16.0),
              TextButton(
                onPressed: () {
                  // Navigate to sign up page
                  Navigator.push(
                    context,
                    MaterialPageRoute(builder: (context) => SignUpScreen()),
                  );
                },
                child: Text('Sign Up'),
              ),
              TextButton(
                onPressed: () {
                  // Navigate to reset password page
                  Navigator.push(
                    context,
                    MaterialPageRoute(builder: (context) => ResetPasswordScreen()),
                  );
                },
                child: Text('Reset Password'),
              ),
            ],
          ),
        ),
      ),
    );
  }

  void _signIn(BuildContext context) async {
    // Validate the form fields
    if (_formKey.currentState!.validate()) {
      // Show a loading indicator while the request is in progress
      showDialog(
        context: context,
        barrierDismissible: false,
        builder: (context) => Center(
          child: CircularProgressIndicator(),
        ),
      );

      // Prepare JSON data for login request
      final Map<String, dynamic> requestData = {
        'username': _usernameController.text,
        'password': _passwordController.text,
      };

      try {
        // Send the POST request
        final response = await http.post(
          Uri.parse('${AppConfig.apiUrl}login'),
          headers: <String, String>{
            'Content-Type': 'application/json; charset=UTF-8',
          },
          body: jsonEncode(requestData),
        );

        // Hide the loading indicator
        Navigator.of(context).pop();

        // Check the response status code
        if (response.statusCode == 200) {
          final Map<String, dynamic> responseData = jsonDecode(response.body);

          if (responseData['success'] == true) {
            // Navigate to the Main Screen if login is successful
            Navigator.pushReplacement(
              context,
              MaterialPageRoute(builder: (context) => MainScreen()),
            );
          } else {
            // Show an error message if login failed
            ScaffoldMessenger.of(context).showSnackBar(
              SnackBar(content: Text('Invalid username or password')),
            );
          }
        } else {
          // Show an error message if server returned an error
          ScaffoldMessenger.of(context).showSnackBar(
            SnackBar(content: Text('Server error, please try again later')),
          );
        }
      } catch (e) {
        // Hide the loading indicator
        Navigator.of(context).pop();

        // Show an error message if there was an exception
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('An error occurred, please try again later')),
        );
      }
    }
  }
}
