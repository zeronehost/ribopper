<template>
  <RiboOptionSection title="操作设置" class="options-view">
    <s-alert>当前匹配模式与剪贴板内容匹配时，可以执行它弹出菜单中的命令。</s-alert>
    <s-card>
      <section class="ribo-cmd">
        <header class="ribo-cmd__row ribo-cmd__header">
          <div class="ribo-cmd__col pattern">匹配模式和命令</div>
          <div class="ribo-cmd__col description">描述</div>
        </header>
        <s-scroll-view class="ribo-cmd__body">
          <RiboOption :options="commands" @selected="selectedHandle" />
        </s-scroll-view>
      </section>
      <footer>
        <div>
          <s-button type="elevated" @click="addHandle">
            <s-icon slot="start" name="add"></s-icon>
            <span>添加操作</span>
          </s-button>
          <s-button type="elevated" :disabled="!selected" @click="editHandle">
            <s-icon slot="start">
              <RiboIconEdit />
            </s-icon>
            <span>编辑操作</span>
          </s-button>
        </div>
        <s-button type="elevated" :disabled="!selected" @click="deleteHandle">
          <s-icon slot="start">
            <RiboIconDelete />
          </s-icon>
          <span>删除操作</span>
        </s-button>
      </footer>
    </s-card>
  </RiboOptionSection>
  
  <s-dialog class="options-view__dialog" :showed="deleteDialogShow">
    <div slot="headline" class="headline">温馨提示</div>
    <div slot="text">确定要删除该操作吗？</div>
    <div slot="action" class="action">
      <s-button type="elevated" @click="deleteConfirmHandle">确定</s-button>
      <s-button type="elevated" @click="deleteCancelHandle">取消</s-button>
    </div>
  </s-dialog>
  <dialog ref="addAndEditDialogEl" class="options-view__dialog-pane">
    <header>添加操作</header>
  </dialog>
</template>
<script setup lang="ts">
import { RiboIconEdit, RiboIconDelete } from "@/components/icons";
import { RiboOptionSection } from "@/components/section";
import { ref } from 'vue';
import { RiboOption } from "@/components/option";
import type { Action } from "@ribo/api";

const commands = ref([
  {
    id: 1,
    name: "NPM Scripts",
    pattern: "npm*",
    description: "Common NPM commands",
    options: [
      {
        command: "npm install <%s>",
        description: "Install a package"
      },
      {
        command: "npm run <%s>",
        description: "Run a script"
      },
      {
        command: "npm start",
        description: "Start the application"
      },
      {
        command: "npm test",
        description: "Run tests"
      }
    ]
  },
  {
    id: 2,
    name: "Git Commands",
    pattern: "git*",
    description: "Version control commands",
    options: [
      {
        command: "git add <%s>",
        description: "Add files to staging area"
      },
      {
        command: "git commit -m '<%s>'",
        description: "Commit with message"
      },
      {
        command: "git push",
        description: "Push changes to remote"
      },
      {
        command: "git pull",
        description: "Pull changes from remote"
      },
      {
        command: "git checkout -b <%s>",
        description: "Create and switch to a new branch"
      }
    ]
  },
  {
    id: 3,
    name: "Docker Commands",
    pattern: "docker*",
    description: "Container management commands",
    options: [
      {
        command: "docker build -t <%s> .",
        description: "Build Docker image"
      },
      {
        command: "docker run -p <%s>:<%s> <%s>",
        description: "Run Docker container with port mapping"
      },
      {
        command: "docker ps",
        description: "List running containers"
      },
      {
        command: "docker-compose up",
        description: "Start services with docker-compose"
      }
    ]
  },
  {
    id: 4,
    name: "File Operations",
    pattern: "file*",
    description: "File and directory operations",
    options: [
      {
        command: "ls -la <%s>",
        description: "List files with details"
      },
      {
        command: "cp -r <%s> <%s>",
        description: "Copy files or directories"
      },
      {
        command: "mv <%s> <%s>",
        description: "Move or rename files"
      },
      {
        command: "find . -name '<%s>'",
        description: "Find files by name"
      }
    ]
  },
  {
    id: 5,
    name: "System Commands",
    pattern: "sys*",
    description: "System management commands",
    options: [
      {
        command: "ps aux | grep <%s>",
        description: "Find running processes"
      },
      {
        command: "kill -9 <%s>",
        description: "Kill a process by ID"
      },
      {
        command: "df -h",
        description: "Check disk space"
      },
      {
        command: "top",
        description: "Monitor system resources"
      }
    ]
  }
]);

const selected = ref<Action>();

const selectedHandle = (data: Action) => {
  selected.value = data;
}

const addAndEditShow = ref(false);
const addHandle = () => {
  addAndEditShow.value = true;
}
const editHandle = () => {
  addAndEditShow.value = true;
}
const addAndEditConfirmHandle = () => {
  addAndEditShow.value = false;
}
const addAndEditCancelHandle = () => {
  addAndEditShow.value = false;
}

const deleteDialogShow = ref(false);
const deleteHandle = () => {
  deleteDialogShow.value = true;
  // TODO: delete action
}
const deleteConfirmHandle = () => {
  deleteDialogShow.value = false;
}
const deleteCancelHandle = () => {
  deleteDialogShow.value = false;
}

</script>
<style lang="scss">
.options-view {
  &.ribo-option-section {
    >s-scroll-view {
      display: flex;
      flex-direction: column;
      gap: 0.5rem;
      overflow: hidden;
    }

    s-alert {
      width: 100%;
    }

    s-card {
      flex: 1;
      margin: 0 0.5rem 0.5rem;
      max-width: unset;
      display: flex;
      flex-direction: column;
      overflow: hidden;
      height: 100%;
    }

    .ribo-cmd {
      flex: 1;
      background-color: var(--s-color-surface-container-lowest);
      overflow: hidden;
      display: flex;
      flex-direction: column;

      &__row {
        display: flex;
      }

      &__col {
        flex: 1;
        padding: 0 0.5rem;
      }

      &__header {
        border-bottom: 1px solid var(--s-color-outline);
        line-height: 2rem;

        .pattern {
          border-right: 1px solid var(--s-color-outline);
        }
      }

      &__body {
        overflow: auto;
      }
    }

    footer {
      display: flex;
      padding: 0.5rem 1rem;
      align-items: center;
      background-color: var(--s-color-surface-container-high);

      s-button {
        border-radius: 6px;
      }

      div {
        flex: 1;

        s-button+s-button {
          margin-left: 0.5rem;
        }
      }
    }
  }

  &__dialog {
    &::part(container) {
      border-radius: 0.5rem;
    }

    .action {
      padding: 0;
    }

    s-button {
      border-radius: 6px;

      &+s-button {
        margin-left: 1rem;
      }
    }

    &-pane {
      border: none;
      padding: 1rem;
      border-radius: 5px;
      outline: none;
      top: 2rem;
      right: 2rem;
      bottom: 2rem;
      left: 2rem;
      width: auto;
      height: auto;
      z-index: 10;

    }
  }
}
</style>