<template>
  <div>
    <div>
      <input v-model="mailAddress" placeholder="メールアドレス">
    </div>
    <button @click="resetPassword()">メールアドレスにパスワードをリセットするリンクを送信</button>
  </div>
</template>

<script lang="ts">
  import firebase from "firebase";
  import {Component, Vue} from "vue-property-decorator";

  import localstorageModule from "@/repository/localstorage/Module"

  const userRepository = localstorageModule.userRepository;

  @Component
  export default class PasswordResetForm extends Vue {

    private mailAddress: string | null = null;

    private resetPassword() {
      if(this.mailAddress != null) {

        // メールリンクでログインする際に使用
        userRepository.setEmail(this.mailAddress);

        const redirectUrl = window.location.origin + this.$route.fullPath;

        firebase.auth()
          .sendPasswordResetEmail(this.mailAddress, {url: redirectUrl})
          .then(() => alert("メールアドレスにパスワードをリセットするリンクを送信しました"))
          .catch(e => {
            console.error(e.toString());
            alert("パスワードをリセットするリンクの送信に失敗しました。入力したメールアドレスが不正な可能性があります。もしくは、メールアドレスを用いたログインの設定がない可能性があります。")
          })
      } else {
        alert("メールアドレスを指定してください");
      }
    }
  }
</script>
