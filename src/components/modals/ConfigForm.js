import './ConfigForm.css';

import { useEffect, useState } from 'react';

import * as sim from '../../../wasm/simulation';
import { useSim } from '../../hooks/useSim';


const defaultConfig = new sim.Config();
const defaultOptions = Object.keys(defaultConfig.intoObject());

const getStepSize = (value) => {
  // Calculates number of decimals in float values to get a suggested step size
  if (value > 1000) {
    return 100;
  } else if (value > 100) {
    return 10;
  } else if (value > 10) {
    return 1;
  }
  const decimals = value.toString().split('.')[1];
  return decimals ? 10 ** -(decimals.split('').findIndex((d) => d !== '0') + 1) : 1;
};

export default function ConfigForm({ handleClose, isNestedConfig }) {
  const { simConfig, setSimConfig, setIsPaused, setStartNewSim } = useSim();
  const [configOptions, setConfigOptions] = useState(Object.keys(simConfig.intoObject()));
  const [config, setConfig] = useState(simConfig.intoObject());
  const [configStepSizes, setConfigStepSizes] = useState(null);

  // Get array of step sizes (prevents step size changing when value reaches an integer)
  useEffect(() => {
    if (configOptions) {
      setConfigStepSizes(configOptions.map((option) => getStepSize(config[option])));
    }
  }, [configOptions]);

  const handleSubmit = () => {
    setIsPaused(true);
    setSimConfig(new sim.Config(config));
    setStartNewSim(true);
    isNestedConfig ? handleClose(true) : handleClose();
  };

  const resetOptions = (current = true) => {
    if (current) {
      setConfigOptions(Object.keys(simConfig.intoObject()));
      setConfig(simConfig.intoObject());
    } else {
      setConfigOptions(defaultOptions);
      setConfig(defaultConfig.intoObject());
    }
  };

  return (
    <>
      <button
        className="btn close"
        onClick={isNestedConfig ? () => handleClose(false) : handleClose}
      >
        X
      </button>
      <div className="config-form">
        <form>
          {config &&
            configStepSizes &&
            configOptions.map((option, i) => {
              return (
                <label key={option}>
                  <span>{option}:</span>
                  <input
                    type="number"
                    onChange={(e) =>
                      setConfig((prevConfig) => ({
                        ...prevConfig,
                        [option]: parseFloat(e.target.value),
                      }))
                    }
                    value={config[option]}
                    min={configStepSizes[i]}
                    step={configStepSizes[i]}
                  />
                </label>
              );
            })}
        </form>
        <div className="button-group">
          {isNestedConfig && (
            <button className="btn" onClick={() => handleClose(false)}>
              Back
            </button>
          )}
          <button className="btn submit" onClick={handleSubmit}>
            Submit
          </button>
        </div>
        <div className="button-group">
          <span>Reset:</span>
          <button className="btn" onClick={resetOptions}>
            Current
          </button>
          <button className="btn" onClick={() => resetOptions(false)}>
            Default
          </button>
        </div>
      </div>
    </>
  );
}
