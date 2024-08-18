import 'package:dio/dio.dart';
import 'package:fallacious_rooster/api/config/confidential_advisors.dart';
import 'package:fallacious_rooster/ui/components/load_or.dart';
import 'package:fallacious_rooster/ui/components/theme_brightness.dart';
import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:multi_select_flutter/multi_select_flutter.dart';

const double formElementVertPadding = 4.0;

class HomeView extends StatefulWidget {
  final Dio client;
  final String host;

  const HomeView({super.key, required this.client, required this.host});

  @override
  State<HomeView> createState() => _HomeViewState();
}

class _HomeViewState extends State<HomeView> {
  final GlobalKey<FormState> _reportForm = GlobalKey();
  final TextEditingController _emailController = TextEditingController();
  final TextEditingController _messageController = TextEditingController();
  bool _allowContacting = false;
  bool _toBoard = false;
  List<ConfidentialAdvisor> _confidentialAdvisors = [];
  String? _contactEmail = null;

  bool _advisorsLoading = true;
  List<ConfidentialAdvisor> _availableConfidentialAdvisors = [];
  String? _boardEmail = null;

  @override
  void initState() {
    super.initState();
    _loadConfidentialAdvisors();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Row(
          children: [
            Image.network(
              "https://public.svsticky.nl/logos/hoofd_outline_wit.png",
              width: 40,
            ),
            Text(AppLocalizations.of(context)!.title),
          ],
        ),
        actions: const [
          ThemeBrightnessButton(),
        ],
      ),
      body: Padding(
        padding: const EdgeInsets.all(8.0),
        child: Center(
          child: Container(
            constraints: const BoxConstraints(maxWidth: 750),
            child: Card(
              child: Padding(
                padding: const EdgeInsets.all(16.0),
                child: Column(
                  children: [
                    Padding(
                      padding: const EdgeInsets.symmetric(vertical: 8.0),
                      child: Text(
                        AppLocalizations.of(context)!.title,
                        style: Theme.of(context).textTheme.headlineMedium,
                      ),
                    ),
                    Text(AppLocalizations.of(context)!.description,
                        style: Theme.of(context).textTheme.bodyMedium),
                    const SizedBox(
                      height: 50,
                    ),
                    Form(
                      key: _reportForm,
                      child: Column(
                        children: [
                          Padding(
                            padding: const EdgeInsets.symmetric(
                                vertical: formElementVertPadding),
                            child: TextFormField(
                              keyboardType: TextInputType.multiline,
                              maxLines: null,
                              minLines: 4,
                              controller: _messageController,
                              validator: _validateRequired,
                              autofocus: true,
                              autovalidateMode:
                                  AutovalidateMode.onUserInteraction,
                              decoration: InputDecoration(
                                  labelText: AppLocalizations.of(context)!
                                      .messageBoxLabel,
                                  alignLabelWithHint: true,
                                  border: const OutlineInputBorder(),
                                  labelStyle:
                                      Theme.of(context).textTheme.bodyMedium,
                                  errorStyle: Theme.of(context)
                                      .textTheme
                                      .bodySmall!
                                      .apply(color: Colors.redAccent)),
                            ),
                          ),
                          Padding(
                            padding: const EdgeInsets.symmetric(
                                vertical: formElementVertPadding),
                            child: TextFormField(
                              controller: _emailController,
                              validator: _validateEmail,
                              autovalidateMode: AutovalidateMode.always,
                              decoration: InputDecoration(
                                  labelText:
                                      AppLocalizations.of(context)!.yourEmail,
                                  labelStyle:
                                      Theme.of(context).textTheme.bodyMedium,
                                  errorStyle: Theme.of(context)
                                      .textTheme
                                      .bodySmall!
                                      .apply(color: Colors.redAccent)),
                            ),
                          ),
                        ],
                      ),
                    ),
                    Padding(
                      padding: const EdgeInsets.symmetric(
                          vertical: formElementVertPadding),
                      child: Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          Text(AppLocalizations.of(context)!.mayContact,
                              style: Theme.of(context).textTheme.bodyMedium),
                          Checkbox(
                            value: _allowContacting,
                            onChanged: (v) => setState(() {
                              _allowContacting = v ?? false;
                            }),
                          ),
                        ],
                      ),
                    ),
                    Padding(
                      padding: const EdgeInsets.symmetric(
                          vertical: formElementVertPadding),
                      child: Row(
                        mainAxisAlignment: MainAxisAlignment.spaceBetween,
                        children: [
                          Text(AppLocalizations.of(context)!.toBoard,
                              style: Theme.of(context).textTheme.bodyMedium),
                          Checkbox(
                            value: _toBoard,
                            onChanged: (v) => setState(() {
                              _toBoard = v ?? false;
                            }),
                          )
                        ],
                      ),
                    ),
                    Row(
                      mainAxisAlignment: MainAxisAlignment.spaceBetween,
                      children: [
                        Text(
                            AppLocalizations.of(context)!
                                .confidentialAdvisorsLabel,
                            style: Theme.of(context).textTheme.bodyMedium),
                        LoaderOr(
                          loading: _advisorsLoading,
                          size: 20,
                          child: MultiSelectDialogField(
                            items: _availableConfidentialAdvisors
                                .map((v) => MultiSelectItem(v, v.name))
                                .toList(),
                            listType: MultiSelectListType.LIST,
                            onConfirm: (vs) => setState(() {
                              _confidentialAdvisors = vs;
                            }),
                            decoration: BoxDecoration(
                              border: Border(
                                  bottom: BorderSide(
                                      color: Theme.of(context)
                                          .buttonTheme
                                          .colorScheme!
                                          .primary,
                                      width: 2)),
                            ),
                            itemsTextStyle:
                                Theme.of(context).textTheme.bodyMedium,
                            selectedItemsTextStyle:
                                Theme.of(context).textTheme.bodyMedium,
                            confirmText: Text(
                                AppLocalizations.of(context)!.selectButtonLabel,
                                style: Theme.of(context).textTheme.bodyLarge),
                            cancelText: Text(
                                AppLocalizations.of(context)!.cancelButtonLabel,
                                style: Theme.of(context).textTheme.bodyLarge),
                            buttonIcon: Icon(
                                color: Theme.of(context).iconTheme.color,
                                Icons.arrow_downward),
                          ),
                        )
                      ],
                    )
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }

  String? _validateRequired(String? v) => (v != null && v.isNotEmpty)
      ? null
      : AppLocalizations.of(context)!.required;

  String? _validateEmail(String? v) {
    if ((v == null || v.isEmpty) && _allowContacting) {
      return AppLocalizations.of(context)!.required;
    } else if (v == null || v.isEmpty) {
      return null;
    }

    bool emailOk = RegExp(
            "(([^<>()\\[\\]\\\\.,;:\\s@\"]+(\\.[^<>()\\[\\]\\\\.,;:\\s@\"]+)*)|(\".+\"))@((\\[[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}\\.[0-9]{1,3}])|(([a-zA-Z\\-0-9]+\\.)+[a-zA-Z]{2,}))")
        .hasMatch(v);
    return emailOk ? null : AppLocalizations.of(context)!.emailInvalid;
  }

  void _loadConfidentialAdvisors() async {
    _advisorsLoading = true;
    List<ConfidentialAdvisor> adv =
        await ConfidentialAdvisors.list(widget.client, widget.host);
    _advisorsLoading = false;

    if (context.mounted) {
      _availableConfidentialAdvisors = adv;
    }
  }
}
