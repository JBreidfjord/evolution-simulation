import './ConfigForm.css';

import { useEffect, useState } from 'react';

import { Config } from '../../../wasm/simulation';
import { useSim } from '../../hooks/useSim';

const defaultConfig = new Config({});
const defaultOptions = Object.keys(defaultConfig.toJSON());

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

interface ConfigFormProps {
  handleClose(isSimReady?: boolean): void,
  isNestedConfig?: boolean
}

export default function ConfigForm({ handleClose, isNestedConfig }: ConfigFormProps) {
  const { simConfig, setSimConfig, setIsPaused, setStartNewSim } = useSim();
  const [configOptions, setConfigOptions] = useState(Object.keys(simConfig.toJSON()));
  const [config, setConfig] = useState(simConfig.toJSON());
  const [configStepSizes, setConfigStepSizes] = useState(null);

  // Get array of step sizes (prevents step size changing when value reaches an integer)
  useEffect(() => {
    if (configOptions) {
      setConfigStepSizes(configOptions.map((option) => getStepSize(config[option])));
    }
  }, [configOptions]);

  const handleSubmit = () => {
    setIsPaused(true);
    setSimConfig(new Config(config));
    setStartNewSim(true);
    isNestedConfig ? handleClose(true) : handleClose();
  };

  const resetOptions = (current) => {
    if (current) {
      setConfigOptions(Object.keys(simConfig.toJSON()));
      setConfig(simConfig.toJSON());
    } else {
      setConfigOptions(defaultOptions);
      setConfig(defaultConfig.toJSON());
    }
  };

  return (
    <>
      <button
        className="btn close"
        onClick={() => isNestedConfig ? handleClose(false) : handleClose}
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
          <button className="btn" onClick={() => resetOptions(true)}>
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
