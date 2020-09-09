const PIXI = require('pixi.js');

import('../pkg').then(rust => {
  const canvas = document.getElementById('canvas');
  const app = new PIXI.Application({
    backgroundColor: 0xffffff,
    view: canvas,
  });

  const state = {
    isRunning: false,
    timeout: 100,
    width: 100,
    height: 50,
    aliveOdds: 0.2,
  };

  const createUniverse = () => {
    let universe = rust.Universe.new(state.width, state.height, state.aliveOdds, app.stage, app.renderer);
    universe.render();
    return universe;
  };

  let universe = createUniverse();
  const cellSize = universe.get_cell_size();

  const renderLoop = () => {
    console.time('RENDER');
    universe.render();
    console.timeEnd('RENDER');

    if (state.isRunning === true) {
      setTimeout(() => {
        console.time('TICK');
        universe.tick();
        console.timeEnd('TICK');
        startLoop();
      }, state.timeout);
    }
  };

  const startLoop = () => requestAnimationFrame(renderLoop);

  document.getElementById('canvas').addEventListener('click', e => {
    universe.click(Math.floor(e.layerX / cellSize), Math.floor(e.layerY / cellSize));
    universe.render();
  });
  document.getElementById('run-pause').addEventListener('click', () => {
    state.isRunning = !state.isRunning;
    let label = 'Run';
    if (state.isRunning === true) {
      startLoop();
      label = 'Pause';
    }
    document.getElementById('run-pause').innerText = label;
  });
  document.getElementById('reset').addEventListener('click', () => {
    state.isRunning = false;
    universe = createUniverse();
  });
  document.getElementById('height').addEventListener('change', e => {
    state.height = parseInt(e.target.value);
    universe = createUniverse();
  });
  document.getElementById('width').addEventListener('change', e => {
    state.width = parseInt(e.target.value);
    universe = createUniverse();
  });
  document.getElementById('alive-odds').addEventListener('change', e => {
    state.aliveOdds = parseFloat(e.target.value);
    universe = createUniverse();
  });
  document.getElementById('timeout').addEventListener('change', e => {
    state.timeout = parseInt(e.target.value);
  });

  document.getElementById('height').value = state.height;
  document.getElementById('width').value = state.width;
  document.getElementById('alive-odds').value = state.aliveOdds;
  document.getElementById('timeout').value = state.timeout;
  startLoop();
});
