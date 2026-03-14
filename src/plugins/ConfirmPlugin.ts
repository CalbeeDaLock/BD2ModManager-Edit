import { App } from 'vue';
import { useConfirm } from './ConfirmService';

export default {
  install(app: App) {
    const { confirm } = useConfirm();
    app.config.globalProperties.$confirm = confirm;
  },
};
