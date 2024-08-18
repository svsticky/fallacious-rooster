import 'package:adaptive_theme/adaptive_theme.dart';
import 'package:fallacious_rooster/authorization.dart';
import 'package:fallacious_rooster/ui/views/home.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:dio/dio.dart';
import 'package:google_fonts/google_fonts.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

import 'dio/dio_adapter_stub.dart'
    if (dart.library.io) 'dio/dio_adapter_mobile.dart'
    if (dart.library.js) 'dio/dio_adapter_web.dart';

const int boardColor = 0x61518f;

void main() {
  runApp(App());
}

class App extends StatefulWidget {
  final Dio dio = _initDio();
  final String host = "http://flutter.localhost:8080";

  App({super.key});

  @override
  State<App> createState() => _AppState();
}

class _AppState extends State<App> {
  @override
  void initState() {
    super.initState();

    if (kIsWeb) {
      Authorization.ensureAuthorized(widget.dio, widget.host);
    }
  }

  @override
  Widget build(BuildContext context) {
    return AdaptiveTheme(
      light: _themeData(Brightness.light),
      dark: _themeData(Brightness.dark),
      initial: AdaptiveThemeMode.dark,
      builder: (theme, darkTheme) => MaterialApp(
        title: 'Fallacious Rooster',
        home: HomeView(client: widget.dio, host: widget.host),
        theme: theme,
        darkTheme: darkTheme,
        localizationsDelegates: AppLocalizations.localizationsDelegates,
        supportedLocales: AppLocalizations.supportedLocales,
        debugShowCheckedModeBanner: false,
      ),
    );
  }
}

ThemeData _themeData(Brightness brightness) {
  ColorScheme scheme =
      ColorScheme.fromSeed(seedColor: _boardColor(), brightness: brightness);
  Color textColor;
  switch (brightness) {
    case Brightness.light:
      {
        textColor = Colors.black;
        break;
      }
    case Brightness.dark:
      {
        textColor = Colors.white;
        break;
      }
  }

  return ThemeData(
      brightness: brightness,
      colorScheme: scheme,
      appBarTheme: AppBarTheme(
          backgroundColor: _boardColor(),
          titleTextStyle: GoogleFonts.oxygen(color: Colors.white),
          iconTheme: IconThemeData(
            color: textColor,
          )),
      useMaterial3: true,
      textTheme: TextTheme(
        headlineMedium: GoogleFonts.oxygen(color: textColor),
      ),
      cardTheme: const CardTheme(elevation: 2));
}

Dio _initDio() {
  BaseOptions options;
  if (kIsWeb) {
    options = BaseOptions(
      validateStatus: (_) => true,
    );
  } else {
    options = BaseOptions(headers: {'X-DebugAllowUnauthorized': 'true'});
  }

  Dio dio = Dio(options);

  if (kIsWeb) {
    HttpClientAdapter adapter = getAdapter();
    dio.httpClientAdapter = adapter;
  }

  return dio;
}

Color _boardColor() {
  int r = (boardColor & 0xFF0000) >> 16;
  int g = (boardColor & 0xFF00) >> 8;
  int b = boardColor & 0xFF;

  return Color.fromARGB(255, r, g, b);
}
