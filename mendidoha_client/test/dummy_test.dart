import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';

void main() {
  testWidgets('Widget Renders Correctly', (WidgetTester tester) async {
    // Build a MaterialApp with a Scaffold containing a Text widget
    await tester.pumpWidget(MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: Text('Test')),
        body: Center(
          child: Text('Hello, World!'),
        ),
      ),
    ));

    // Verify that the Text widget with 'Hello, World!' text is present
    expect(find.text('Hello, World!'), findsOneWidget);

    // Tap on the Text widget
    await tester.tap(find.text('Hello, World!'));
    await tester.pump();

    // Verify that tapping on the Text widget triggers the expected behavior
    expect(find.text('You tapped the button!'), findsNothing);
  });
}
