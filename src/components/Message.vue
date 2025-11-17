<template>
  <TransitionRoot as="template" :show="visible">
    <div class="fixed inset-0 flex items-end justify-center p-4 z-50 pointer-events-none">
      <TransitionChild
        as="template"
        enter="ease-out duration-300"
        enter-from="opacity-0 translate-y-2"
        enter-to="opacity-100 translate-y-0"
        leave="ease-in duration-200"
        leave-from="opacity-100 translate-y-0"
        leave-to="opacity-0 translate-y-2"
      >
        <div
          class="pointer-events-auto rounded-lg bg-white shadow-xl ring-1 ring-gray-900/5 px-6 py-4 w-full sm:w-auto sm:max-w-md"
          :class="typeClasses"
        >
          <div class="flex items-center gap-3">
            <div
              class="flex h-10 w-10 flex-shrink-0 items-center justify-center rounded-full"
              :class="iconContainerClass"
            >
              <component
                :is="iconComponent"
                class="h-6 w-6"
                :class="iconClass"
                aria-hidden="true"
              />
            </div>
            <div class="flex-1">
              <p class="text-sm font-medium" :class="textClass">
                {{ message }}
              </p>
            </div>
            <button
              @click="close"
              class="text-gray-400 hover:text-gray-500 flex-shrink-0"
            >
              <span class="sr-only">Close</span>
              <XMarkIcon class="h-5 w-5" aria-hidden="true" />
            </button>
          </div>
        </div>
      </TransitionChild>
    </div>
  </TransitionRoot>
</template>

<script lang="ts">
import { ref, computed } from "vue";
import { TransitionChild, TransitionRoot } from "@headlessui/vue";
import {
  CheckCircleIcon,
  ExclamationTriangleIcon,
  InformationCircleIcon,
  XCircleIcon,
  XMarkIcon,
} from "@heroicons/vue/24/outline";

type MessageType = "success" | "error" | "warning" | "info";

export default {
  props: {
    message: String,
    type: {
      type: String as () => MessageType,
      default: "info",
      validator: (value: string) => ["success", "error", "warning", "info"].includes(value),
    },
    duration: {
      type: Number,
      default: 3000,
    },
  },
  components: {
    TransitionRoot,
    TransitionChild,
    CheckCircleIcon,
    ExclamationTriangleIcon,
    InformationCircleIcon,
    XCircleIcon,
    XMarkIcon,
  },

  setup(props) {
    const visible = ref(false);
    let timeoutId: ReturnType<typeof setTimeout> | null = null;

    const iconComponent = computed(() => {
      switch (props.type) {
        case "success":
          return "CheckCircleIcon";
        case "error":
          return "XCircleIcon";
        case "warning":
          return "ExclamationTriangleIcon";
        case "info":
        default:
          return "InformationCircleIcon";
      }
    });

    const iconContainerClass = computed(() => {
      switch (props.type) {
        case "success":
          return "bg-green-100";
        case "error":
          return "bg-red-100";
        case "warning":
          return "bg-yellow-100";
        case "info":
        default:
          return "bg-blue-100";
      }
    });

    const iconClass = computed(() => {
      switch (props.type) {
        case "success":
          return "text-green-600";
        case "error":
          return "text-red-600";
        case "warning":
          return "text-yellow-600";
        case "info":
        default:
          return "text-blue-600";
      }
    });

    const textClass = computed(() => {
      switch (props.type) {
        case "success":
          return "text-green-900";
        case "error":
          return "text-red-900";
        case "warning":
          return "text-yellow-900";
        case "info":
        default:
          return "text-blue-900";
      }
    });

    const typeClasses = computed(() => {
      switch (props.type) {
        case "success":
          return "bg-green-50";
        case "error":
          return "bg-red-50";
        case "warning":
          return "bg-yellow-50";
        case "info":
        default:
          return "bg-blue-50";
      }
    });

    const show = () => {
      visible.value = true;
      if (timeoutId) clearTimeout(timeoutId);
      timeoutId = setTimeout(close, props.duration);
    };

    const close = () => {
      visible.value = false;
      if (timeoutId) clearTimeout(timeoutId);
    };

    return {
      visible,
      show,
      close,
      iconComponent,
      iconContainerClass,
      iconClass,
      textClass,
      typeClasses,
    };
  },
};
</script>
