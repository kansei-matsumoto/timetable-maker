import { invoke } from "@tauri-apps/api/core";

window.addEventListener("DOMContentLoaded", () => {
  
  // サイドバーの制御
  const sidebarContents = document.getElementById("sidebar-contents") as HTMLElement;
  const sidebarToggle = document.getElementById("sidebar-toggle") as HTMLElement;

  sidebarToggle.addEventListener("click", () => {
    sidebarContents.classList.toggle("active");
    if(sidebarContents.classList.contains("active")){
      sidebarToggle.textContent="×";
    }else{
      sidebarToggle.textContent="|||";
    }
  });

  // モーダル画面の制御
  document.querySelectorAll(".modal-button").forEach(modalButton=>{
    const dialog = document.getElementById("modal-"+modalButton.id) as HTMLDialogElement;
    modalButton.addEventListener("click", () => {
      dialog.showModal();
    });
    document.getElementById("close-"+modalButton.id)?.addEventListener("click",() => {
      dialog.close();
    });
  });

  // Excelのインポート
  document.querySelector("#import-button")?.addEventListener("click",()=>{
    importExcel();
  });
});

async function importExcel() {
  const result = await invoke("import_excel", {
    // pwd:src-tauri
    xlPath: "./template_01.xlsm",
  });
  console.log(result);
}


