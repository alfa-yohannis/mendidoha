import 'package:flutter/material.dart';
import 'package:http/http.dart' as http;
import 'dart:convert';
import 'sign_in_screen.dart'; // Import the SignInScreen widget
import 'reset_password_success.dart'; // Import the ResetSuccessScreen widget

class ResetPasswordScreen extends StatelessWidget {
  final _formKey = GlobalKey<FormState>();
  final TextEditingController _usernameController = TextEditingController();
  final TextEditingController _codeController = TextEditingController();
  final TextEditingController _newPasswordController = TextEditingController();
  final TextEditingController _confirmPasswordController = TextEditingController();

  // Regular expression for password validation
  final String passwordPattern = r'^(?=.*[A-Za-z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$';

  Future<void> _updatePassword(BuildContext context, String code, String username, String newPassword) {
    final url = Uri.parse('http://0.0.0.0:8080/reset_password'); // Updated API endpoint
    return http.post(
      url,
      headers: {
        'Content-Type': 'application/json',
      },
      body: json.encode({
        'username': username,
        'reset_code': code,
        'new_password': newPassword,
      }),
    ).then((response) {
      if (response.statusCode == 200) {
        // If the server returns a 200 OK response, navigate to the reset success screen
        Navigator.pushReplacement(
          context,
          MaterialPageRoute(builder: (context) => ResetSuccessScreen()),
        );
      } else {
        // If the server returns an error, throw an exception
        print('Failed to update password: ${response.body}');
        throw Exception('Failed to update password');
      }
    }).catchError((error) {
      // Handle any errors from http post request
      print('Error during password update: $error');
      throw Exception('Failed to update password');
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Reset Password'),
      ),
      body: Padding(
        padding: const EdgeInsets.all(16.0),
        child: Form(
          key: _formKey,
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: <Widget>[
              TextFormField(
                controller: _usernameController,
                decoration: InputDecoration(
                  labelText: 'Username',
                  border: OutlineInputBorder(),
                ),
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter your username';
                  }
                  return null;
                },
              ),
              SizedBox(height: 16.0),
              TextFormField(
                controller: _codeController,
                decoration: InputDecoration(
                  labelText: 'Reset Code',
                  border: OutlineInputBorder(),
                ),
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter the reset code';
                  }
                  return null;
                },
              ),
              SizedBox(height: 16.0),
              TextFormField(
                controller: _newPasswordController,
                obscureText: true,
                decoration: InputDecoration(
                  labelText: 'New Password',
                  border: OutlineInputBorder(),
                ),
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please enter your new password';
                  }
                  // Add password validation logic here if needed
                  return null;
                },
              ),
              SizedBox(height: 16.0),
              TextFormField(
                controller: _confirmPasswordController,
                obscureText: true,
                decoration: InputDecoration(
                  labelText: 'Confirm Password',
                  border: OutlineInputBorder(),
                ),
                validator: (value) {
                  if (value == null || value.isEmpty) {
                    return 'Please confirm your new password';
                  } else if (value != _newPasswordController.text) {
                    return 'Passwords do not match';
                  }
                  return null;
                },
              ),
              SizedBox(height: 16.0),
              ElevatedButton(
                onPressed: () async {
                  // Validate the form fields
                  if (_formKey.currentState!.validate()) {
                    String username = _usernameController.text;
                    String code = _codeController.text;
                    String newPassword = _newPasswordController.text;
                    String confirmPassword = _confirmPasswordController.text;
                    print('Username: $username, Reset Code: $code, New Password: $newPassword, Confirm Password: $confirmPassword');
                    
                    try {
                      // Call the _updatePassword function and handle navigation in a then block
                      await _updatePassword(context, code, username, newPassword);
                    } catch (e) {
                      // Handle error
                      ScaffoldMessenger.of(context).showSnackBar(
                        SnackBar(content: Text('Failed to update password')),
                      );
                    }
                  }
                },
                child: Text('Update Password'),
              ),
              SizedBox(height: 16.0),
              ElevatedButton(
                onPressed: () {
                  // Navigate back to the sign-in screen
                  Navigator.pushReplacement(
                    context,
                    MaterialPageRoute(builder: (context) => SignInScreen()),
                  );
                },
                child: Text('Back to Sign In'),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
