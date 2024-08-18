import 'package:flutter/material.dart';

class StickyCardFooter extends StatelessWidget {
  const StickyCardFooter({super.key});

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisAlignment: MainAxisAlignment.center,
      children: [
        SizedBox(
            width: 30,
            height: 30,
            child: Image.network(
                "https://public.svsticky.nl/logos/hoofd_outline_wit.png")),
        const Text("Studievereniging Sticky"),
      ],
    );
  }
}
