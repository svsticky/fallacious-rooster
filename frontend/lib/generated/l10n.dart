// GENERATED CODE - DO NOT MODIFY BY HAND
import 'package:flutter/material.dart';
import 'package:intl/intl.dart';
import 'intl/messages_all.dart';

// **************************************************************************
// Generator: Flutter Intl IDE plugin
// Made by Localizely
// **************************************************************************

// ignore_for_file: non_constant_identifier_names, lines_longer_than_80_chars
// ignore_for_file: join_return_with_assignment, prefer_final_in_for_each
// ignore_for_file: avoid_redundant_argument_values, avoid_escaping_inner_quotes

class S {
  S();

  static S? _current;

  static S get current {
    assert(_current != null,
        'No instance of S was loaded. Try to initialize the S delegate before accessing S.current.');
    return _current!;
  }

  static const AppLocalizationDelegate delegate = AppLocalizationDelegate();

  static Future<S> load(Locale locale) {
    final name = (locale.countryCode?.isEmpty ?? false)
        ? locale.languageCode
        : locale.toString();
    final localeName = Intl.canonicalizedLocale(name);
    return initializeMessages(localeName).then((_) {
      Intl.defaultLocale = localeName;
      final instance = S();
      S._current = instance;

      return instance;
    });
  }

  static S of(BuildContext context) {
    final instance = S.maybeOf(context);
    assert(instance != null,
        'No instance of S present in the widget tree. Did you add S.delegate in localizationsDelegates?');
    return instance!;
  }

  static S? maybeOf(BuildContext context) {
    return Localizations.of<S>(context, S);
  }

  /// `Anonymous Reporting Point`
  String get title {
    return Intl.message(
      'Anonymous Reporting Point',
      name: 'title',
      desc: '',
      args: [],
    );
  }

  /// `Welcome to Sticky's anonymous reporting point. You can report unwanted behavior here anonymously, but also other things that bother you. You can send the report to the board or to the confidential advisors.`
  String get description {
    return Intl.message(
      'Welcome to Sticky\'s anonymous reporting point. You can report unwanted behavior here anonymously, but also other things that bother you. You can send the report to the board or to the confidential advisors.',
      name: 'description',
      desc: '',
      args: [],
    );
  }

  /// `Required`
  String get required {
    return Intl.message(
      'Required',
      name: 'required',
      desc: '',
      args: [],
    );
  }

  /// `Your message`
  String get messageBoxLabel {
    return Intl.message(
      'Your message',
      name: 'messageBoxLabel',
      desc: '',
      args: [],
    );
  }

  /// `May we contact you? (Your email address will then be included)`
  String get mayContact {
    return Intl.message(
      'May we contact you? (Your email address will then be included)',
      name: 'mayContact',
      desc: '',
      args: [],
    );
  }

  /// `Your email address, if you want us to contact you`
  String get yourEmail {
    return Intl.message(
      'Your email address, if you want us to contact you',
      name: 'yourEmail',
      desc: '',
      args: [],
    );
  }

  /// `Invalid email address`
  String get emailInvalid {
    return Intl.message(
      'Invalid email address',
      name: 'emailInvalid',
      desc: '',
      args: [],
    );
  }

  /// `Should your message be delivered to the board?`
  String get toBoard {
    return Intl.message(
      'Should your message be delivered to the board?',
      name: 'toBoard',
      desc: '',
      args: [],
    );
  }

  /// `To which confidential advisors should your message me send?`
  String get confidentialAdvisorsLabel {
    return Intl.message(
      'To which confidential advisors should your message me send?',
      name: 'confidentialAdvisorsLabel',
      desc: '',
      args: [],
    );
  }

  /// `Select`
  String get selectButtonLabel {
    return Intl.message(
      'Select',
      name: 'selectButtonLabel',
      desc: '',
      args: [],
    );
  }

  /// `Cancel`
  String get cancelButtonLabel {
    return Intl.message(
      'Cancel',
      name: 'cancelButtonLabel',
      desc: '',
      args: [],
    );
  }
}

class AppLocalizationDelegate extends LocalizationsDelegate<S> {
  const AppLocalizationDelegate();

  List<Locale> get supportedLocales {
    return const <Locale>[
      Locale.fromSubtags(languageCode: 'en'),
      Locale.fromSubtags(languageCode: 'nl'),
    ];
  }

  @override
  bool isSupported(Locale locale) => _isSupported(locale);
  @override
  Future<S> load(Locale locale) => S.load(locale);
  @override
  bool shouldReload(AppLocalizationDelegate old) => false;

  bool _isSupported(Locale locale) {
    for (var supportedLocale in supportedLocales) {
      if (supportedLocale.languageCode == locale.languageCode) {
        return true;
      }
    }
    return false;
  }
}
