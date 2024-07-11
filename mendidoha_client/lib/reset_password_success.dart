import 'package:flutter/material.dart';

class ResetSuccessScreen extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('Password Reset Complete'),
      ),
      body: Center(
        child: Padding(
          padding: const EdgeInsets.all(16.0),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: <Widget>[
              Text(
                'Your password has been successfully reset!',
                style: TextStyle(fontSize: 18.0),
                textAlign: TextAlign.center,
              ),
              SizedBox(height: 20.0),
              ElevatedButton(
                onPressed: () {
                  // Navigate back to the sign-in screen
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
