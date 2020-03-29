export default class UserRepository {

  private readonly key = "email";

  public findEmail(): string | null {
    const mailAddress =  window.localStorage.getItem(this.key);

    if(!mailAddress) {
      console.warn("MailAddress is null");
    }

    return mailAddress;
  }

  public setEmail(email: string) {
    console.info("MailAddress is set to local storage, mailAddress = " + email);
    window.localStorage.setItem(this.key, email);
  }

  public findEmailOrElse(defaultValue: string): string {
    const mailAddress = this.findEmail();
    return mailAddress ? mailAddress : defaultValue;
  }
}
