import 'package:adaptive_theme/adaptive_theme.dart';
import 'package:flutter/material.dart';

class ThemeBrightnessButton extends StatelessWidget {
  const ThemeBrightnessButton({super.key});

  @override
  Widget build(BuildContext context) {
    return IconButton(
      icon: Icon(() {
        switch (Theme.of(context).brightness) {
          case Brightness.light:
            return Icons.dark_mode;
          case Brightness.dark:
            return Icons.light_mode;
        }
      }(), color: Colors.white),
      onPressed: () => AdaptiveTheme.of(context).toggleThemeMode(),
    );
  }
}
