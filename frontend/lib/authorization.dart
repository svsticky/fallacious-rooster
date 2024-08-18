import 'dart:convert';

import 'package:dio/dio.dart';
import 'package:flutter/material.dart';
import 'package:universal_html/html.dart' as html;

class AuthorizationResult {
  final bool ok;
  final bool admin;

  AuthorizationResult(this.ok, this.admin);
}

class Authorization {
  static Future<AuthorizationResult> ensureAuthorized(
      Dio client, String host) async {
    AuthorizationResult ok = await _ok(client, host);
    if (ok.ok) {
      return ok;
    }

    debugPrint("Authorization not OK");

    try {
      Response response = await client.get('$host/api/oauth/login');
      if (response.statusCode == 200) {
        Map<String, dynamic> payload = response.data;
        html.window.location.href = payload['location'];
        return AuthorizationResult(false, false);
      }

      return AuthorizationResult(false, false);
    } on Exception catch (e) {
      debugPrint("Exception: $e");
      return AuthorizationResult(false, false);
    }
  }

  static Future<AuthorizationResult> _ok(Dio client, String host) async {
    try {
      Response isAuth = await client.get("$host/api/oauth/ok");

      if (isAuth.statusCode == 200) {
        Map<String, dynamic> body = jsonDecode(isAuth.data.toString());

        return AuthorizationResult(true, body['is_admin']);
      } else {
        return AuthorizationResult(false, false);
      }
    } on Exception catch (e) {
      debugPrint("Exception: $e");
      return AuthorizationResult(false, false);
    }
  }
}
