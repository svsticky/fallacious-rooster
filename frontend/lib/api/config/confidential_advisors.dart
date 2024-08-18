import 'package:dio/dio.dart';
import 'package:flutter/cupertino.dart';

class ConfidentialAdvisor {
  final String name;
  final String email;

  ConfidentialAdvisor({required this.name, required this.email});
}

class ConfidentialAdvisors {
  static Future<List<ConfidentialAdvisor>> list(Dio client, String host) async {
    try {
      Response r = await client.get("$host/api/config/confidential-advisors");
      Map<String, dynamic> p = r.data;
      List<dynamic> advs = p['advisors'];
      return advs
          .map((adv) =>
              ConfidentialAdvisor(name: adv['name'], email: adv['email']))
          .toList();
    } on Exception catch (e) {
      debugPrint("Exception: $e");
      return [];
    }
  }
}
