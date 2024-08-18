// DO NOT EDIT. This is code generated via package:intl/generate_localized.dart
// This is a library that provides messages for a nl locale. All the
// messages from the main program should be duplicated here with the same
// function name.

// Ignore issues from commonly used lints in this file.
// ignore_for_file:unnecessary_brace_in_string_interps, unnecessary_new
// ignore_for_file:prefer_single_quotes,comment_references, directives_ordering
// ignore_for_file:annotate_overrides,prefer_generic_function_type_aliases
// ignore_for_file:unused_import, file_names, avoid_escaping_inner_quotes
// ignore_for_file:unnecessary_string_interpolations, unnecessary_string_escapes

import 'package:intl/intl.dart';
import 'package:intl/message_lookup_by_library.dart';

final messages = new MessageLookup();

typedef String MessageIfAbsent(String messageStr, List<dynamic> args);

class MessageLookup extends MessageLookupByLibrary {
  String get localeName => 'nl';

  final messages = _notInlinedMessages(_notInlinedMessages);
  static Map<String, Function> _notInlinedMessages(_) => <String, Function>{
        "cancelButtonLabel": MessageLookupByLibrary.simpleMessage("Annuleren"),
        "confidentialAdvisorsLabel": MessageLookupByLibrary.simpleMessage(
            "Naar welke vertrouwenscontactpersonen wil je dat je bericht gestuurd word?"),
        "description": MessageLookupByLibrary.simpleMessage(
            "Welkom bij het anonieme meldpunt van Sticky. Je kunt hier anoniem een melding maken van ongewenst gedrag, maar ook van andere zaken die je dwars liggen. Je kunt de melding sturen naar het bestuur of naar de contactpersonen."),
        "emailInvalid":
            MessageLookupByLibrary.simpleMessage("Ongeldig email adres"),
        "mayContact": MessageLookupByLibrary.simpleMessage(
            "Mogen we contact met je opnemen? (Je email adres wordt dan meegestuurd)"),
        "messageBoxLabel": MessageLookupByLibrary.simpleMessage("Jouw bericht"),
        "required": MessageLookupByLibrary.simpleMessage("Vereist"),
        "selectButtonLabel": MessageLookupByLibrary.simpleMessage("Selecteer"),
        "title": MessageLookupByLibrary.simpleMessage(""),
        "toBoard": MessageLookupByLibrary.simpleMessage(
            "Wil je dat jouw bericht naar het bestuur gaat?"),
        "yourEmail": MessageLookupByLibrary.simpleMessage(
            "Jouw email adres, als je wilt dat we contact met je opnemen")
      };
}
