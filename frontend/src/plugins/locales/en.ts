import {Locale} from "@/plugins/locales/locale";

const EN: Locale = {
  site_title: "Reporting point for intolerant behaviour",
  error: "Something went wrong, please try again later",
  home: {
    welcome: {
      title: "Welcome to the reporting point for intolerant behaviour for Sticky",
      subtitle: "You can make fully anonymous reports here about of unwanted, transgressive, threatening, humiliating or intimidating behavior. You can send this report to (one of the) confidential advisors or to the board, that is completely up to you. Furthermore, you can choose if you want us to contact you if you would like to hear back from us, of course, this is not required",
    },
    form: {
      title: "Describe your report",
      subtitle: "Describe the experience you had or the report you would like to write. You may include names if you want. It may be as long and detailed as you want.",
      message: "Describe your experience or report",
      invalid: "One or multiple fields were not filled in correctly",
      invalidEmail: "The given email adres is not valid",
      required: "Vereist",
      submit: "Send your report",
      toReceivers: "To who do you want your report to be sent?",
      selectReceivers: "Select...",
      allowContact: "Do you want us to contact you about your report?",
      contactEmail: "Email address",
      contactEmailExplanation: "The email address on which you want us to contact you regarding your report. Leave this field blank if you want to remain anonymous",
      selectRecipient: "Your report needs at least one recipient",
      board: "The board"
    },
    success: "Your report has been sent and will be processed"
  }

}

export default EN;