import 'package:flutter/material.dart';

class SignUpNotificationScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Sign Up Complete'),
      ),
      body: Center(
        child: Padding(
          padding: const EdgeInsets.all(16.0),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: <Widget>[
              Text(
                'Your account has been successfully created!',
                style: TextStyle(fontSize: 18.0),
                textAlign: TextAlign.center,
              ),
              SizedBox(height: 20.0),
              ElevatedButton(
                onPressed: () {
                  // Navigate back to the login screen
                  Navigator.popUntil(context, (route) => route.isFirst);
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
