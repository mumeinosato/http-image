<template>
  <div>
    <form @submit.prevent="savePage">
      <!-- URL入力フィールド -->
      <label for="url">URL:</label>
      <input type="text" v-model="url" id="url" placeholder="URLを入力してください" />

      <!-- フォルダ名入力フィールド -->
      <label for="folderName">フォルダ名:</label>
      <input type="text" v-model="folderName" id="folderName" placeholder="フォルダ名を入力してください" />

      <!-- 送信ボタン -->
      <button type="submit">保存</button>
    </form>

    <!-- 成功メッセージ -->
    <div v-if="message">
      <p>{{ message }}</p>
    </div>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api/tauri';

export default {
  data() {
    return {
      url: '',        // ユーザーが入力したURL
      folderName: '', // ユーザーが指定したフォルダ名
      message: ''     // サーバーからのメッセージ
    };
  },
  methods: {
    async savePage() {
      try {
        // Tauriのコマンドを呼び出してバックエンド処理を実行
        const response = await invoke('save_page_content', { url: this.url, folderName: this.folderName });
        this.message = response.message;
      } catch (error) {
        this.message = error.message || 'エラーが発生しました。';
      }
    }
  }
}
</script>
