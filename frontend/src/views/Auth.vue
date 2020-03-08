<template>
  <div>
    <h1>ユーザー新規登録 & ログイン</h1>

    <div>
      <GoogleLoginButton />
    </div>

    <div>
      <MailAddressLoginButton />
    </div>

    <div>
      <MailAddressForgetLoginButton />
    </div>

    <div>
      <LogoutButton />
    </div>
  </div>
</template>

<script lang="ts">
  import {Component, Vue} from "vue-property-decorator";
  import GoogleLoginForm from "@/components/auth/GoogleLoginForm.vue";
  import LogoutForm from "@/components/auth/LogoutForm.vue";
  import MailAddressLoginForm from "@/components/auth/MailAddressLoginForm.vue";
  import MailAddressForgetLoginButton from "@/components/auth/PasswordResetForm.vue";

  @Component({
    components: {MailAddressForgetLoginButton, MailAddressLoginButton: MailAddressLoginForm, LogoutButton: LogoutForm, GoogleLoginButton: GoogleLoginForm}
  })
  export default class Auth extends Vue {}
</script>

<style scoped="scoped">
  div {
    margin: 20px;
  }
</style>
