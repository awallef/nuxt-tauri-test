<script lang="ts" setup>
import { open } from '@tauri-apps/api/dialog';
import { slotFlagsText } from '@vue/shared';

// COMPOSABLE
const {logDebugInfo} = useLog();
const {zipFile} = useZip();

const openDialog = async (evt:any) => {
    try{
        // SELECT FILE
        const selected = await open({
            multiple: false,
            title: 'open file bro'
        });

        if (Array.isArray(selected)) {
            return;
        } else if (selected === null) {
            return;
        } else {
            zip(selected);
        }

    }catch(err)
    {
        logDebugInfo(err as string);
    }
}

const zip = async (path:string) => {

    // LOG FILE PATH
    logDebugInfo(path);
    
    // ZIP FILE
    const outputPath: string = `${path}.zip`;
    await zipFile(path, outputPath);

    // LOG FILE PATH ZIPPED
    logDebugInfo(outputPath);
}
</script>

<template>
  <form action="">
    <button id="openFile" type="button" @click="openDialog">open</button>
  </form>
</template>