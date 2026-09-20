**NickTonks - SEESTAR Transfer & Stacking Tool (v1.0)**
An all-in-one HTML Application (.hta) utility designed for astrophotographers using the Seestar smart telescope series. 

This tool automates telescope node discovery, SMB file transfers, real-time RTSP video monitoring, live weather tracking, and advanced CLI image stacking via Siril.   

**Features & Modules**

**1. Telescope Node DiscoveryAutomatic Subnet Sweep:** Scans your local network range (1–254) to locate active Seestar telescope nodes via ping and DNS hostnames.   Manual IP Input: Allows direct IP validation and connection for Direct Wi-Fi setups or static DHCP reservations.   SMB Guest Authentication: Automatically manages Windows LanmanWorkstation registry policies to ensure seamless guest share access (\\<IP>\EMMC Images).   

**2. File Organization & IngestionAutomated Structure Hierarchy:** Ingests and organizes files directly from telescope units using Windows Robocopy into a structured directory layout:   
[Destination Directory]\[Target Name]\Lights (.fit)
[Destination Directory]\[Target Name]\Videos (.avi/.mp4)
[Destination Directory]\[Target Name]\Jpegs (.jpg)
[Destination Directory]\Seestar_[X].log (Individual transfer logs)

Transfer Controls: Start, pause, and resume sync workflows at any time with duplicate-skip protection.  

**3. Live Stream Monitors (VLC RTSP Integration)**
Built-in multi-grid stream slots (up to 4 simultaneous feeds) leveraging the VLC ActiveX plugin (rtsp://[Device-IP]:4554/stream) to monitor live target acquisition in real time. 

**5. Scientific Observatory Weather DashboardOpen-Meteo Integration:**
Automatically detects public IP location coordinates via IP-API and queries real-time atmospheric data.   Metrics Tracked: Cloud cover percentage, ambient temperature, relative humidity (dew point risk), wind speed, visibility, and estimated Bortle / SQM sky quality indices.   Forecasts: Includes both an hourly cloud/temperature trend tracker (next 24 hours) and a 7-day atmospheric forecast grid.   

**7. Siril CLI Advanced Stacking & FITS PreviewAutomated Batch Processing**
Automatically generates and executes stacking scripts for Siril 1.3.6 (siril-cli.exe), handling plate-solving, calibration, registration, and additive scaling alignment for both standard targets and mosaics.   Built-in FITS Canvas Previewer: Instantly preview raw or stacked .fit/.fits files with non-destructive linear view modes, adjustable auto-stretch intensity sliders, and real-time header metadata inspection.  

**8. Additional UtilitiesNight Vision Mode:**
Toggleable red-tinted UI theme to preserve night-adapted vision at the observatory.   Registry Clean-up: Option on exit to revert Windows insecure guest authentication settings back to system defaults.   

System RequirementsOperating System: Microsoft Windows (Windows 10 / 11 recommended).   Runtime: HTML Application host (mshta.exe), built-in Windows Script Host (WScript.Shell), and PowerShell.   Optional Dependencies:VLC Media Player: Required for viewing live RTSP camera feeds.   Siril (v1.3.6+): Required for utilizing the CLI advanced stacking workspace (C:\Program Files\Siril\bin\siril-cli.exe).   

**Author: Nick Tonks** | Instagram: @NICKTONKS_ASTROPHOTOGRAPHY   
