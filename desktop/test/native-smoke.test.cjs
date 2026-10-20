const test = require('node:test');
const assert = require('node:assert/strict');
const native = require('../numi-module/index.node');

test('native engine evaluates a unit conversion', async () => {
  const model = native.numi_model_new();
  native.numi_model_start.call(model);
  await new Promise(resolve => setTimeout(resolve, 100));
  const output = await native.numi_model_calc.call(model, JSON.stringify({ input: ['2 meters in cm'], options: { precision: 2 } }));
  assert.equal(JSON.parse(output).result[0].value, '200 cm');
});
