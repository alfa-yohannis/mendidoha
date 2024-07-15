import 'package:flutter/material.dart';
import 'package:mendidoha_client/main.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:mendidoha_client/sign_in/sign_in_screen.dart'; // Import your SignInScreen widget

void main() {
  testWidgets('SignInScreen Widget Test', (WidgetTester tester) async {
    // Build the widget and trigger a frame
    await tester.pumpWidget(MendidohaClient());

    // Verify that the SignInScreen is rendered
    expect(find.byType(SignInScreen), findsOneWidget);

    // Example: Verify that there are two text fields
    expect(find.byType(TextFormField), findsNWidgets(2)); // Adjust the count based on your actual widget structure

    // Example: Verify that the 'Username (Email)' text field is present
    expect(find.widgetWithText(TextFormField, 'Username (Email)'), findsOneWidget);

    // Example: Verify that the 'Password' text field is present
    expect(find.widgetWithText(TextFormField, 'Password'), findsOneWidget);
  });
}
