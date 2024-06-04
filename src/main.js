document.addEventListener('DOMContentLoaded', () => {
  const inputFile = document.getElementById('inputFile');
  const outputFile = document.getElementById('outputFile');
  const headerFile = document.getElementById('headerFile');
  const message = document.getElementById('message');

  const invoke = window.__TAURI__.invoke;

  async function handleConvertToSBR() {
      try {
        await invoke('convert_to_sbr', {
            inputFile: inputFile.value,
            outputFile: outputFile.value,
            headerFile: headerFile.value
        });
        message.textContent = 'Conversion to SBR complete';
    } catch (error) {
        message.textContent = `Error: ${error}`;
    }
}

async function handleConvertFromSBR() {
    try {
        await invoke('convert_from_sbr', {
            inputFile: inputFile.value,
            headerFile: headerFile.value,
            outputFile: outputFile.value
        });
        message.textContent = 'Conversion from SBR complete';
    } catch (error) {
        message.textContent = `Error: ${error}`;
    }
}

async function handleConvertToHBMP() {
    try {
        await invoke('convert_to_hbmp', {
            inputFile: inputFile.value,
            outputFile: outputFile.value,
            headerFile: headerFile.value
        });
        message.textContent = 'Conversion to headless BMP complete';
    } catch (error) {
        message.textContent = `Error: ${error}`;
    }
}

async function handleConvertFromHBMP() {
    try {
        await invoke('convert_from_hbmp', {
            inputFile: inputFile.value,
            headerFile: headerFile.value,
            outputFile: outputFile.value
        });
        message.textContent = 'Conversion from headless BMP complete';
    } catch (error) {
        message.textContent = `Error: ${error}`;
    }
}

document.getElementById('convertToSBR').addEventListener('click', handleConvertToSBR);
document.getElementById('convertFromSBR').addEventListener('click', handleConvertFromSBR);
document.getElementById('convertToHBMP').addEventListener('click', handleConvertToHBMP);
document.getElementById('convertFromHBMP').addEventListener('click', handleConvertFromHBMP);
});
