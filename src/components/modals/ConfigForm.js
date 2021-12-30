import "./ConfigForm.css";

import * as sim from "../../../build/lib_simulation_wasm";

import { useEffect, useState } from "react";

import { useSim } from "../../hooks/useSim";

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
  const decimals = value.toString().split(".")[1];
  return decimals ? 10 ** -(decimals.split("").findIndex((d) => d !== "0") + 1) : 1;
};

export default function ConfigForm({ handleClose }) {
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
    handleClose();
  };

  useEffect(() => {
    console.log(simConfig);
  }, [simConfig]);

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
    <div className="config-form">
      <button className="btn close" onClick={handleClose}>
        X
      </button>
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
      <button className="btn submit" onClick={handleSubmit}>
        Submit
      </button>
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
  );
}