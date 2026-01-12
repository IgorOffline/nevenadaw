(async function () {
  async function decemberInit() {
    console.log("START::decemberInit");
    const div = document.getElementById('december-pixi');
    if (div) {
      const app = new PIXI.Application();
      await app.init({
        background: '#455A64',
        resizeTo: div
      });
      div.appendChild(app.canvas);
    } else {
      console.error("The element with ID 'december-pixi' was not found.");
    }

    console.log("END::decemberInit");
  }

  if (document.readyState === 'loading') {
    window.addEventListener('DOMContentLoaded', async () => {
      await decemberInit();
    });
  } else {
    await decemberInit();
  }
})();