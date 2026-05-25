# Battery Simulator

## Overview

A standalone battery simulator that models multiple batteries with realistic physics, command handling, and state persistence. It maintains its own database (separate from any parent system) and is fully self-contained.

## Simulation Loop

```
Tick Service (1s)               Flush Service (5s)
       |                              |
       |  read from cache             |  drain dirty set
       v                              v
  Simulation.Tick()             Repository.Flush()
       |                              |
       |  mark dirty                  |  bulk write to database
       v                              v
  DirtyTracker                  DbContext.SaveChanges()
```


## Simulation Physics

### SOC Calculation

```text
deltaSoc = (power * efficiency * elapsedHours) / ratedCapacity
         - selfDischarge * elapsedHours
```

| Status    | Power Sign | Efficiency |
|-----------|------------|------------|
| Charging  | positive   | 95% (5% loss) |
| Discharging | negative | ~105% (5% loss) |
| Idle      | zero       | 100% |

### Power Control

- **Ramping**: Power changes linearly at set rate toward target (simulates inverter response)
- **Idle drift**: When idle with non-zero power, decays toward zero
- **SOC limits**: Automatically stops charging at max SOC and discharging at min SOC
- **Self-discharge**: Constant small loss regardless of state

### Command Persistence

A Charge or Discharge command stays active until:
- SOC hits a limit (max/min) — command is cleared
- A Hold command is received — power ramps to zero
- A new command overrides it

## API Endpoints

### Battery Management

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/batteries` | List all simulated batteries |
| `POST` | `/api/batteries` | Create a new simulated battery |

**POST /api/batteries** — request body:
```json
{
  "name": "Warehouse Battery #1",
  "ratedCapacityKwh": 100,
  "maxChargePowerKw": 50,
  "maxDischargePowerKw": 50
}
```

Returns `201 Created` with auto-generated device ID.

### Telemetry

| Method | Path | Description |
|--------|------|-------------|
| `GET` | `/api/health/{deviceId}` | Battery health and telemetry |
| `GET` | `/api/heartbeat/{deviceId}` | Battery heartbeat |

**GET /api/health/{deviceId}** — response:
```json
{
  "status": "ok",
  "timestamp": "2026-05-25T12:00:00Z",
  "deviceId": "3f14a315-81df-4d84-90c7-7bbe11cb5ae4",
  "powerKw": 25.5,
  "soc": 72.3,
  "firmware": "sim-1.0.0",
  "uptimeSeconds": 3600
}
```

**GET /api/heartbeat/{deviceId}** — response:
```json
{
  "status": "alive",
  "timestamp": "2026-05-25T12:00:00Z",
  "deviceId": "3f14a315-81df-4d84-90c7-7bbe11cb5ae4",
  "intervalSeconds": 30
}
```

### Commands

| Method | Path | Description |
|--------|------|-------------|
| `POST` | `/api/command` | Send a command to a battery |

**POST /api/command** — request body:
```json
{
  "deviceId": "3f14a315-81df-4d84-90c7-7bbe11cb5ae4",
  "commandType": "Charge",
  "targetPowerKw": 25.0
}
```

| commandType | Behavior |
|-------------|----------|
| `Charge` | Ramp power to positive target (clamped to max) |
| `Discharge` | Ramp power to negative target (clamped to max) |
| `Hold` | Ramp power to zero |
| `SetPowerTarget` | Ramp to any value within power limits |

Response:
```json
{
  "success": true,
  "deviceId": "3f14a315-81df-4d84-90c7-7bbe11cb5ae4",
  "powerKw": 25.0,
  "soc": 72.3,
  "status": "Charging",
  "commandType": "Charge",
  "targetPowerKw": 25.0
}
```