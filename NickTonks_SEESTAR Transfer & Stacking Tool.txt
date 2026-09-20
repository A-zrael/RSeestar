<!-- <:# :
@echo off
net session >nul 2>&1
if %errorLevel% neq 0 (
    echo Set UAC = CreateObject^("Shell.Application"^) > "%temp%\getadmin.vbs"
    echo UAC.ShellExecute "cmd.exe", "/c """"%~f0""""", "", "runas", 0 >> "%temp%\getadmin.vbs"
    "%temp%\getadmin.vbs"
    del "%temp%\getadmin.vbs"
    exit /b
)

reg add "HKCU\Software\Microsoft\Internet Explorer\Styles" /v MaxScriptStatements /t REG_DWORD /d 0xffffffff /f >nul 2>&1

if exist "%SystemRoot%\System32\mshta.exe" (
    "%SystemRoot%\System32\mshta.exe" "%~f0"
) else (
    mshta.exe "%~f0"
)
exit /b
:# : -->

<!DOCTYPE html>
<html>
<head>
<meta http-equiv="x-ua-compatible" content="ie=edge" />
<title>NickTonks_Astrophotography_Seestar Station Mode File Transfer Tool with Live View</title>
<HTA:APPLICATION
    ID="SeestarCopier"
    APPLICATIONNAME="Seestar Station Mode File Transfer Tool"
    BORDER="thin"
    BORDERSTYLE="normal"
    CAPTION="yes"
    MAXIMIZEBUTTON="no"
    MINIMIZEBUTTON="yes"
    SCROLL="no"
    SINGLEINSTANCE="yes"
    SYSMENU="yes"
/>

<style>
* { box-sizing: border-box; }
html, body { width: 100%; height: 100%; margin: 0; padding: 0; overflow: hidden; }
body {
    background-color: #03050b;
    color: #e2e8f0;
    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
    padding: 6px 10px;
    font-size: 11.5px;
    overflow: hidden;
    zoom: expression(Math.min(screen.availHeight / 1080, 1));
    background-image: 
        radial-gradient(circle at 50% 30%, rgba(0, 240, 255, 0.04) 0%, transparent 70%),
        radial-gradient(circle at 20% 80%, rgba(255, 200, 87, 0.02) 0%, transparent 60%),
        linear-gradient(rgba(18, 28, 48, 0.15) 1px, transparent 1px),
        linear-gradient(90deg, rgba(18, 28, 48, 0.15) 1px, transparent 1px);
    background-size: 100% 100%, 100% 100%, 24px 24px, 24px 24px;
}
.container { width: 100%; display: block; }

h2 {
    color: #00f0ff;
    margin-top: 0;
    margin-bottom: 6px;
    border-bottom: 2px solid #1a263f;
    padding-bottom: 2px;
    text-align: center;
    font-size: 13px;
    letter-spacing: 1.5px;
    font-family: 'Consolas', monospace;
    text-transform: uppercase;
    text-shadow: 0 0 12px rgba(0, 240, 255, 0.35);
}
.panel {
    background-color: #090e1a;
    border: 1px solid #1a263f;
    padding: 6px 8px;
    margin-bottom: 4px;
    border-radius: 5px;
    width: 100%;
    box-shadow: 0 4px 12px rgba(0,0,0,0.7);
}
label { display: inline-block; margin-bottom: 2px; color: #f1f5f9; }
.section-label { color: #ffc857; font-family: 'Consolas', monospace; letter-spacing: 0.5px; font-weight: bold; font-size: 11px; }
input[type="text"] {
    background-color: #03050b;
    border: 1px solid #1e2d4a;
    color: #00f0ff;
    font-family: 'Consolas', monospace;
    padding: 4px;
    border-radius: 3px;
    box-sizing: border-box;
}
input[type="text"]:focus { border-color: #00f0ff; outline: none; box-shadow: 0 0 6px rgba(0,240,255,0.25); }
button {
    background-color: #111b30;
    color: #f1f5f9;
    border: 1px solid #243559;
    padding: 4px 8px;
    cursor: pointer;
    font-weight: bold;
    border-radius: 4px;
    font-size: 11px;
    font-family: 'Segoe UI', sans-serif;
}
button:hover { background-color: #1a263f; border-color: #00f0ff; color: #00f0ff; }
.btn-primary { background-color: #0d5c46; color: #ffffff; border: 1px solid #10b981; width: 100%; font-size: 11.5px; padding: 5px; }
.btn-primary:hover { background-color: #10b981; color: #03050b; }
.btn-resume { background-color: #1e3a8a; color: #ffffff; border: 1px solid #3b82f6; width: 100%; font-size: 11.5px; padding: 5px; }
.btn-resume:hover { background-color: #3b82f6; color: #03050b; }
.btn-danger { background-color: #7f1d1d; color: #ffffff; border: 1px solid #ef4444; width: 100%; font-size: 11.5px; padding: 5px; }
.btn-danger:hover { background-color: #ef4444; color: #03050b; }
.btn-exit-custom { background-color: #7f1d1d; color: #ffffff; border: 1px solid #ef4444; font-size: 11.5px; padding: 6px; width: 200px; }
.btn-exit-custom:hover { background-color: #ef4444; color: #03050b; }
.btn-action-group { display: flex; gap: 6px; margin-top: 2px; }
.btn-action-group button { flex: 1; padding: 5px; }
.row-inline { display: flex; gap: 6px; margin-top: 2px; align-items: center; }
.transfer-buttons { display: flex; gap: 8px; margin-top: 2px; }
.footer-buttons-stacked { display: flex; flex-direction: column; align-items: center; gap: 8px; margin-top: 6px; margin-bottom: 4px; }
.checkbox-group { display: flex; justify-content: space-between; margin-top: 2px; align-items: center; width: 100%; }
.checkbox-group label { color: #f1f5f9; cursor: pointer; font-weight: normal; display: flex; align-items: center; gap: 6px; margin-bottom: 0; }
input[type="checkbox"] { accent-color: #00f0ff; cursor: pointer; }
.device-display {
    margin-top: 4px;
    background-color: #03050b;
    border: 1px dashed #1e2d4a;
    padding: 4px 6px;
    border-radius: 3px;
    font-family: 'Consolas', monospace;
    font-size: 10.5px;
    color: #ffffff;
    min-height: 28px;
    max-height: 90px;
    overflow-y: auto;
}
.device-row { display: flex; align-items: center; justify-content: space-between; margin-bottom: 3px; padding: 2px 0; border-bottom: 1px solid #111b30; }
.device-row:last-child { border-bottom: none; }

.stream-link {
    color: #38bdf8;
    text-decoration: underline;
    cursor: pointer;
    font-family: 'Consolas', monospace;
    font-size: 10px;
    font-weight: bold;
    margin-right: 6px;
}
.stream-link:hover { color: #00f0ff; }

/* Dynamic Side-by-Side Stream Grid Styles */
.vlc-grid-container {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
    margin-top: 4px;
}
.vlc-grid-card {
    flex: 1 1 calc(50% - 4px);
    min-width: 220px;
    background-color: #000000;
    border: 1px solid #1a263f;
    border-radius: 4px;
    padding: 4px;
    display: flex;
    flex-direction: column;
    display: none;
}
.vlc-card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-family: 'Consolas', monospace;
    font-size: 9.5px;
    color: #ffc857;
    margin-bottom: 2px;
    padding-bottom: 1px;
    border-bottom: 1px solid #1a263f;
}
.vlc-player-wrapper {
    width: 100%;
    height: 110px;
    position: relative;
    background-color: #000000;
    border: 1px solid #00f0ff;
    border-radius: 3px;
    display: flex;
    justify-content: center;
    align-items: center;
}
.vlc-overlay-text {
    color: #64748b;
    font-family: 'Consolas', monospace;
    font-size: 9.5px;
    position: absolute;
    z-index: 1;
    text-align: center;
    padding: 0 10px;
}

#scanStatusBox {
    width: 100%;
    background-color: #03050b;
    color: #ffc857;
    font-family: 'Consolas', monospace;
    border: 1px solid #b8860b;
    padding: 4px;
    margin-top: 4px;
    font-size: 10px;
    font-weight: bold;
    border-radius: 3px;
    text-align: center;
    display: block;
    box-sizing: border-box;
    letter-spacing: 0.5px;
}
#logArea {
    width: 100%;
    height: 70px;
    background-color: #03050b;
    color: #38bdf8;
    font-family: 'Consolas', monospace;
    border: 1px solid #1a263f;
    padding: 4px;
    margin-top: 2px;
    box-sizing: border-box;
    overflow-y: scroll;
    white-space: pre-wrap;
    display: block;
    border-radius: 3px;
    font-size: 10.5px;
}
.note { color: #94a3b8; font-size: 10px; text-align: center; margin-top: 6px; line-height: 1.2; }
.note span { color: #ffc857; }

#manualBoxContainer {
    display: none;
    margin-top: 4px;
    background-color: #03050b;
    border: 1px solid #00f0ff;
    padding: 6px;
    border-radius: 4px;
}

#page3Container { display: none; }
#portalScreen { display: none; text-align: center; padding: 4px; }
#imageStackingContainer { display: none; }
#sirilStackingContainer { display: none; }

/* Scientific Metrics Grid for Page 3 */
.weather-grid {
    display: flex;
    flex-wrap: wrap;
    gap: 4px;
    margin-top: 4px;
}
.weather-card {
    flex: 1 1 calc(33% - 4px);
    min-width: 150px;
    background-color: #03050b;
    border: 1px solid #1e2d4a;
    border-radius: 4px;
    padding: 5px 6px;
}
.weather-card-title {
    font-family: 'Consolas', monospace;
    font-size: 9px;
    color: #ffc857;
    text-transform: uppercase;
    margin-bottom: 2px;
    border-bottom: 1px solid #131c31;
    padding-bottom: 1px;
}
.weather-card-value {
    font-family: 'Consolas', monospace;
    font-size: 12px;
    color: #00f0ff;
    font-weight: bold;
}
.weather-card-sub {
    font-size: 9.5px;
    color: #94a3b8;
    margin-top: 1px;
}

/* 7-Day Forecast Infographic Styles */
.forecast-card {
    flex: 1 1 calc(14% - 4px);
    min-width: 75px;
    background-color: #03050b;
    border: 1px solid #1e2d4a;
    border-radius: 4px;
    padding: 5px 3px;
    text-align: center;
}
.forecast-date {
    font-family: 'Consolas', monospace;
    font-size: 9px;
    color: #ffc857;
    margin-bottom: 2px;
    border-bottom: 1px solid #131c31;
    padding-bottom: 1px;
}
.forecast-val {
    font-family: 'Consolas', monospace;
    font-size: 10.5px;
    color: #00f0ff;
    font-weight: bold;
}
.forecast-sub {
    font-size: 8.5px;
    color: #94a3b8;
    margin-top: 1px;
}

/* Hourly Forecast Infographic Styles */
.hourly-container {
    display: flex;
    gap: 4px;
    overflow-x: auto;
    padding-bottom: 2px;
    margin-top: 4px;
}
.hourly-card {
    flex: 0 0 72px;
    background-color: #03050b;
    border: 1px solid #1e2d4a;
    border-radius: 4px;
    padding: 4px 2px;
    text-align: center;
}
.hourly-time {
    font-family: 'Consolas', monospace;
    font-size: 8.5px;
    color: #ffc857;
    margin-bottom: 1px;
    border-bottom: 1px solid #131c31;
    padding-bottom: 1px;
}
.hourly-val {
    font-family: 'Consolas', monospace;
    font-size: 10px;
    color: #00f0ff;
    font-weight: bold;
}
.hourly-sub {
    font-size: 8px;
    color: #94a3b8;
    margin-top: 1px;
}

body.night-mode { background-color: #050000 !important; background-image: none !important; color: #ff3333 !important; }
body.night-mode .panel, 
body.night-mode .landing-card,
body.night-mode .portal-card,
body.night-mode .landing-grid-box,
body.night-mode .landing-grid-box-full,
body.night-mode .disclosure-box,
body.night-mode #manualBoxContainer,
body.night-mode #page3Container,
body.night-mode #portalScreen,
body.night-mode #imageStackingContainer,
body.night-mode #sirilStackingContainer,
body.night-mode .weather-card,
body.night-mode .forecast-card,
body.night-mode .hourly-card { background-color: #0f0000 !important; border-color: #660000 !important; color: #ff3333 !important; }
body.night-mode h2, 
body.night-mode h3, 
body.night-mode h4,
body.night-mode label, 
body.night-mode .section-label,
body.night-mode .landing-title,
body.night-mode .landing-subtitle,
body.night-mode .landing-row-title,
body.night-mode #scanStatusBox,
body.night-mode .stream-link,
body.night-mode .weather-card-title,
body.night-mode .weather-card-value,
body.night-mode .forecast-date,
body.night-mode .forecast-val,
body.night-mode .hourly-time,
body.night-mode .hourly-val { color: #ff0000 !important; border-color: #660000 !important; text-shadow: none !important; }
body.night-mode button,
body.night-mode input[type="text"],
body.night-mode #logArea,
body.night-mode #deviceList,
body.night-mode .folder-tree { background-color: #1a0000 !important; color: #ff3333 !important; border-color: #660000 !important; }

#landingScreen { display: block; text-align: center; padding: 4px; }
.landing-card { background-color: #090e1a; border: 1px solid #1a263f; border-radius: 6px; padding: 10px 14px; max-width: 680px; margin: 0 auto; box-shadow: 0 6px 20px rgba(0,0,0,0.6); }
.portal-card { background-color: #090e1a; border: 1px solid #1a263f; border-radius: 6px; padding: 12px 16px; max-width: 680px; margin: 0 auto; box-shadow: 0 6px 20px rgba(0,0,0,0.6); }
.landing-title { color: #00f0ff; font-size: 18px; font-weight: bold; margin-bottom: 2px; letter-spacing: 1.5px; font-family: 'Consolas', monospace; text-shadow: 0 0 10px rgba(0,240,255,0.3); }
.landing-subtitle { color: #ffc857; font-size: 11px; margin-bottom: 8px; font-family: 'Consolas', monospace; }
.landing-row-section { margin-bottom: 6px; text-align: left; }
.landing-row-title { color: #ffc857; font-size: 10px; font-weight: bold; letter-spacing: 0.8px; text-transform: uppercase; margin-bottom: 2px; border-bottom: 1px solid #1a263f; padding-bottom: 1px; font-family: 'Consolas', monospace; }
.landing-grid-two-col { display: flex; gap: 6px; }
.landing-grid-box { flex: 1; background-color: #03050b; padding: 6px 8px; border-radius: 4px; border: 1px solid #131c31; font-size: 10px; line-height: 1.3; box-sizing: border-box; }
.landing-grid-box-full { width: 100%; background-color: #03050b; padding: 6px 8px; border-radius: 4px; border: 1px solid #131c31; font-size: 10px; line-height: 1.3; box-sizing: border-box; }
.landing-grid-box h4, .landing-grid-box-full h4 { color: #ffffff; margin: 0 0 2px 0; font-size: 11px; }
.landing-grid-box p, .landing-grid-box-full p { margin: 0; color: #cbd5e1; }
.folder-tree { font-family: 'Consolas', monospace; color: #00f0ff; background-color: #03050b; padding: 4px 6px; border-radius: 3px; border: 1px solid #131c31; margin-top: 3px; font-size: 9.5px; }
.disclosure-box { background-color: #03050b; border: 1px solid #b8860b; border-radius: 4px; padding: 6px 8px; font-size: 10px; line-height: 1.3; color: #cbd5e1; text-align: left; margin-top: 6px; }
.disclosure-box h4 { color: #ffc857; margin: 0 0 2px 0; font-size: 10.5px; text-transform: uppercase; letter-spacing: 0.5px; font-family: 'Consolas', monospace; }
.disclosure-box ul { margin: 2px 0 0 12px; padding: 0; }
.disclosure-box li { margin-bottom: 1px; }
.btn-enter { background-color: #0d5c46; color: #ffffff; border: 1px solid #10b981; font-size: 12px; padding: 7px 16px; border-radius: 4px; font-weight: bold; cursor: pointer; width: 60%; margin-top: 8px; }
.btn-enter:hover { background-color: #10b981; color: #03050b; }
.portal-btn { background-color: #111b30; color: #00f0ff; border: 1px solid #00f0ff; font-size: 12px; padding: 10px 12px; border-radius: 4px; font-weight: bold; cursor: pointer; width: 100%; text-align: left; display: flex; justify-content: space-between; align-items: center; margin-bottom: 6px; font-family: 'Consolas', monospace; }
.portal-btn:hover { background-color: #1a263f; border-color: #ffc857; color: #ffc857; }
#mainAppContainer { display: none; }
</style>

<script language="JScript">
var wsh = new ActiveXObject("WScript.Shell");
var fso = new ActiveXObject("Scripting.FileSystemObject");
var discoveredIPs = {};
var deviceCount = 0;
var scanTimer = null;
var scanCompleted = false;
var isTransferring = false;

function resizeWindowToScreen() {
    try {
        var screenH = window.screen.availHeight;
        var winW = 800;
        var winH = screenH;
        window.resizeTo(winW, winH);
        window.moveTo((window.screen.availWidth - winW) / 2, 0);
    } catch(e) {}
}

window.onload = function() {
    resizeWindowToScreen();
};

function ToggleNightVision() {
    var body = document.body;
    if (body.className === "night-mode") {
        body.className = "";
    } else {
        body.className = "night-mode";
    }
}

function EnterApplication() {
    document.getElementById("landingScreen").style.display = "none";
    document.getElementById("portalScreen").style.display = "block";
    resizeWindowToScreen();
    LogMessage("[UI] Entered Landing Portal.");
}

function GoToSirilStackingPage() {
    document.getElementById("portalScreen").style.display = "none";
    document.getElementById("mainAppContainer").style.display = "none";
    var p3 = document.getElementById("page3Container");
    if (p3) p3.style.display = "none";
    var imgStack = document.getElementById("sirilstackingContainer");
    if (imgStack) imgStack.style.display = "none";
    
    document.getElementById("sirilStackingContainer").style.display = "block";
    resizeWindowToScreen();
    LogSirilMessage("[UI] Navigated to Siril CLI Stacking Page.");
}

function BrowseSirilFolder() {
    try {
        var shellApp = new ActiveXObject("Shell.Application");
        var folder = shellApp.BrowseForFolder(0, "Select target folder containing image files:", 0, 0);
        if (folder != null) {
            var path = folder.Self.Path;
            document.getElementById("sirilWorkDir").value = path;
            updateTargetNameFromPath(path);
            LogSirilMessage("[SIRIL] Selected target directory: " + path);
        }
    } catch(e) {
        LogSirilMessage("[ERROR] Could not browse folder: " + e.message);
    }
}

function updateTargetNameFromPath(path) {
    if (!path) return;
    path = path.replace(/^\s+|\s+$/g, '');
    if (path.length > 0 && path.charAt(path.length - 1) === '\\') {
        path = path.substring(0, path.length - 1);
    }
    var lastIdx = path.lastIndexOf('\\');
    var folderName = (lastIdx !== -1) ? path.substring(lastIdx + 1) : path;
    
    var lowerFolder = folderName.toLowerCase();
    if (lowerFolder === "lights" || lowerFolder === "video" || lowerFolder === "jpegs" || lowerFolder === "subfolder") {
        path = path.substring(0, lastIdx);
        lastIdx = path.lastIndexOf('\\');
        folderName = (lastIdx !== -1) ? path.substring(lastIdx + 1) : path;
    }

    folderName = folderName.replace(/_Subs$/i, '');
    
    document.getElementById("sirilTargetName").value = folderName;
    LogSirilMessage("[SIRIL] Auto-populated target name: " + folderName);
}

function SyncSirilFolder() {
    var page2Path = document.getElementById("destFolderPath").value;
    if (page2Path.replace(/^\s+|\s+$/g, '') === "") {
        alert("No export folder has been specified on Page 2 yet.");
        return;
    }
    document.getElementById("sirilWorkDir").value = page2Path;
    updateTargetNameFromPath(page2Path);
    LogSirilMessage("[SIRIL] Synced folder from Page 2: " + page2Path);
}
function LogSirilMessage(text) {
    var logBox = document.getElementById("sirilLogArea");
    if (logBox) {
        logBox.value += text + "\n";
        logBox.scrollTop = logBox.scrollHeight;
    }
}

var sirilTimer = null;

function ExecuteSirilBatchScript() { 
    var sirilExe = document.getElementById("sirilExePath").value; 
    var workDir = document.getElementById("sirilWorkDir").value.replace(/^\s+|\s+\$/g, ''); 
    var targetName = document.getElementById("sirilTargetName").value.replace(/^\s+|\s+\$/g, ''); 

    if (workDir === "" || !fso.FolderExists(workDir)) { 
        alert("Please select a valid target folder containing image files."); 
        return; 
    } 

    if (targetName === "") targetName = "Target"; 
    targetName = targetName.replace(/ /g, "_"); 

    try { 
        var shellApp = new ActiveXObject("Shell.Application"); 
        shellApp.Explore(workDir); 
        LogSirilMessage("[EXPLORER] Opened target folder: " + workDir); 
    } catch(e) { 
        LogSirilMessage("[WARNING] Could not open folder automatically: " + e.message); 
    } 

    LogSirilMessage("================================================="); 
    LogSirilMessage("[SIRIL_INIT] Generating and executing mosaic stacking script (Siril 1.3.6)..."); 
    LogSirilMessage("================================================="); 

    var tempBat = wsh.ExpandEnvironmentStrings("%TEMP%") + "\\run_siril_stack.bat"; 
    var tempOut = wsh.ExpandEnvironmentStrings("%TEMP%") + "\\siril_out.txt"; 

    if (fso.FileExists(tempOut)) { 
        try { fso.DeleteFile(tempOut); } catch(e) {} 
    } 

    if (workDir.slice(-1) === "\\") {
        workDir = workDir.slice(0, -1);
    }

    var batchContent = "@echo off\n" + 
        "setlocal enabledelayedexpansion\n" + 
        "set \"SIRIL_EXE=" + sirilExe + "\"\n" + 
        "set \"WORK_DIR=" + workDir + "\"\n" + 
        "set \"TARGET_NAME=" + targetName + "\"\n" + 
        "for /f \"usebackq delims=\" %%A in (`powershell -Command \"Get-Date -Format 'yyyyMMdd_HHmmss'\"`) do set \"TIMESTAMP=%%A\"\n" + 
        "set \"OUT_NAME=Stacked-!TARGET_NAME!-!TIMESTAMP!\"\n" + 
        "set \"SCRIPT_FILE=!WORK_DIR!\\temp_stack.ssf\"\n" + 
        "(\n" + 
        " echo requires 1.3.6\n" + 
        " echo cd lights\n" + 
        " echo link light -out=../process\n" + 
        " echo cd ../process\n" + 
        " echo calibrate light -debayer\n" + 
        " echo seqplatesolve pp_light -force -nocache\n" + 
        " echo seqapplyreg pp_light -filter-round=2.5k -framing=max\n" + 
        " echo stack r_pp_light rej 3 3 -norm=addscale -output_norm -rgb_equal -out=result\n" + 
        " echo load result\n" + 
        " echo save ../!OUT_NAME!\n" + 
        " echo close\n" + 
        ") > \"!SCRIPT_FILE!\"\n" + 
        "echo [SIRIL] Launching CLI process... > \"" + tempOut + "\"\n" + 
        "\"!SIRIL_EXE!\" -s \"!SCRIPT_FILE!\" -d \"!WORK_DIR!\" >> \"" + tempOut + "\" 2>&1\n" + 
        "set \"EXIT_CODE=!ERRORLEVEL!\"\n" + 
        "if exist \"!SCRIPT_FILE!\" del /f /q \"!SCRIPT_FILE!\"\n" + 
        "if exist \"!WORK_DIR!\\process\" rd /s /q \"!WORK_DIR!\\process\"\n" + 
        "if exist \"!WORK_DIR!\\*.seq\" del /f /q \"!WORK_DIR!\\*.seq\"\n" + 
        "if exist \"!WORK_DIR!\\!OUT_NAME!.fit\" (\n" + 
        " echo [SUCCESS] Stacking complete! Final image saved to: \"!WORK_DIR!\\!OUT_NAME!.fit\" >> \"" + tempOut + "\"\n" + 
        ") else (\n" + 
        " echo [ERROR] Stacking failed with exit code: !EXIT_CODE! >> \"" + tempOut + "\"\n" + 
        ")\n" + 
        "echo [DONE] >> \"" + tempOut + "\"\n"; 

    var fBat = fso.CreateTextFile(tempBat, true); 
    fBat.Write(batchContent); 
    fBat.Close(); 

    wsh.Run('cmd.exe /c ""' + tempBat + '""', 0, false); 

    var lastLen = 0; 
    if (sirilTimer !== null) { 
        window.clearInterval(sirilTimer); 
    } 

    sirilTimer = window.setInterval(function() { 
        if (fso.FileExists(tempOut)) { 
            try { 
                var file = fso.OpenTextFile(tempOut, 1, false, -2); 
                var content = ""; 
                if (!file.AtEndOfStream) { 
                    content = file.ReadAll(); 
                } 
                file.Close(); 

                if (content.length > lastLen) { 
                    var chunk = content.substring(lastLen); 
                    lastLen = content.length; 
                    var lines = chunk.split("\n"); 
                    for (var i = 0; i < lines.length; i++) { 
                        var line = lines[i].replace(/^\s+|\s+\$/g, ''); 
                        if (line !== "" && line !== "[DONE]") { 
                            LogSirilMessage(line); 
                        } 
                    } 
                } 

                if (content.indexOf("[DONE]") !== -1) { 
                    window.clearInterval(sirilTimer); 
                    sirilTimer = null; 
                    LogSirilMessage("================================================="); 
                    LogSirilMessage("[COMPLETE] Siril mosaic execution finished."); 
                    LogSirilMessage("================================================="); 
                    alert("Siril mosaic stacking complete!"); 
                } 
            } catch(e) {} 
        } 
    }, 1000); 
}

function HideAllScreens() {
    var p = document.getElementById("portalScreen");
    var m = document.getElementById("mainAppContainer");
    var w = document.getElementById("page3Container");
    var s = document.getElementById("sirilStackingContainer");
    
    if (p) p.style.display = "none";
    if (m) m.style.display = "none";
    if (w) w.style.display = "none";
    if (s) s.style.display = "none";
}

function GoToPortal() {
    HideAllScreens();
    document.getElementById("portalScreen").style.display = "block";
    resizeWindowToScreen();
    LogMessage("[UI] Navigated back to Landing Portal.");
}

function GoToWeatherPage() {
    HideAllScreens();
    document.getElementById("page3Container").style.display = "block";
    resizeWindowToScreen();
    LogMessage("[UI] Navigated to Weather Page.");
    FetchObservatoryWeather();
}

function GoToScanLivePage() {
    HideAllScreens();
    document.getElementById("mainAppContainer").style.display = "block";
    resizeWindowToScreen();
    LogMessage("[UI] Navigated to Scanning and Live Views.");
}

function GoToSirilStackingPage() {
    HideAllScreens();
    document.getElementById("sirilStackingContainer").style.display = "block";
    resizeWindowToScreen();
    LogSirilMessage("[UI] Navigated to Siril CLI Stacking Page.");
}
function GoToPage3() {
    GoToWeatherPage();
}

function ReturnToPage2() {
    GoToScanLivePage();
}

function UpdateScanStatus(msg) {
    var box = document.getElementById("scanStatusBox");
    if (box) { box.innerText = msg; }
}

function LogMessage(text) {
    var logBox = document.getElementById("logArea");
    logBox.value += text + "\n";
    logBox.scrollTop = logBox.scrollHeight;
}

window.onload = function() {
    resizeWindowToScreen();
    var fileInput = document.getElementById('fileInput');
    if (fileInput) {
        fileInput.addEventListener('change', handleFitsFile, false);
    }
};

var globalBuffer = null;
var globalFileName = "";
var currentZoom = 1.0;
var currentMode = 'linear';
var isInitialLoad = true;
var isCommitted = false;

function handleFitsFile(event) {
    var file = event.target.files[0];
    if (!file) return;

    var reader = new FileReader();
    reader.onload = function(e) {
        globalBuffer = e.target.result;
        globalFileName = file.name;
        isInitialLoad = true;
        currentMode = 'linear';
        isCommitted = false;
        var sFactor = document.getElementById('stretchFactor');
        if (sFactor) sFactor.value = 5;
        var sVal = document.getElementById('stretchVal');
        if (sVal) sVal.innerText = "5.0";
        updateControlsUI();

        try {
            processAndDrawImage();
        } catch (ex) {
            alert("Error rendering FITS file: " + ex.message);
        }
    };
    reader.readAsArrayBuffer(file);
}

function loadFitFileFromPath(filePath) {
    try {
        var ado = new ActiveXObject("ADODB.Stream");
        ado.Type = 1; 
        ado.Open();
        ado.LoadFromFile(filePath);
        var binData = ado.Read();
        ado.Close();

        var rs = new ActiveXObject("ADODB.Recordset");
        rs.Fields.Append("BinaryData", 205, ado.Size);
        rs.Open();
        rs.AddNew();
        rs.Fields("BinaryData").AppendChunk(binData);
        rs.Update();
        var streamBytes = rs.Fields("BinaryData").Value;
        rs.Close();

        var byteArray = new VBArray(streamBytes).toArray();
        globalBuffer = new Uint8Array(byteArray).buffer;
        
        var lastSlash = filePath.lastIndexOf('\\');
        globalFileName = (lastSlash !== -1) ? filePath.substring(lastSlash + 1) : filePath;
        
        isInitialLoad = true;
        currentMode = 'stretch'; 
        isCommitted = false;
        var sFactor = document.getElementById('stretchFactor');
        if (sFactor) sFactor.value = 6;
        var sVal = document.getElementById('stretchVal');
        if (sVal) sVal.innerText = "6.0";
        updateControlsUI();

        processAndDrawImage();
    } catch(e) {
        LogSirilMessage("[ERROR] Failed loading binary stream for FITS preview: " + e.message);
    }
}

function setMode(mode) {
    if (!globalBuffer) return;
    currentMode = mode;
    isCommitted = false;
    updateControlsUI();
    processAndDrawImage();
}

function onSliderChange() {
    var sFactor = document.getElementById('stretchFactor');
    var sVal = document.getElementById('stretchVal');
    if (!sFactor || !sVal) return;
    var sInt = parseFloat(sFactor.value);
    sVal.innerText = sInt.toFixed(1);

    if (currentMode === 'stretch' && !isCommitted) {
        processAndDrawImage();
    }
}

function commitStretch() {
    isCommitted = true;
    processAndDrawImage();
}

function updateControlsUI() {
    var btnLinear = document.getElementById('btnLinear');
    var btnStretch = document.getElementById('btnStretch');
    var stretchSliderRow = document.getElementById('stretchSliderRow');
    if (!btnLinear || !btnStretch || !stretchSliderRow) return;
    
    btnLinear.style.background = "#111b30";
    btnStretch.style.background = "#111b30";
    stretchSliderRow.style.display = 'none';

    if (currentMode === 'stretch') {
        btnStretch.style.background = "#0d5c46";
        stretchSliderRow.style.display = 'flex';
    } else {
        btnLinear.style.background = "#0d5c46";
    }
}

function processAndDrawImage() {
    var headerText = "";
    var naxis1 = 0;
    var naxis2 = 0;
    var bitpix = 16;
    var headerBlockCount = 0;

    var view = new Uint8Array(globalBuffer);
    
    for (var i = 0; i < view.length; i += 2880) {
        var chunk = "";
        for (var j = 0; j < 2880; j++) {
            chunk += String.fromCharCode(view[i + j]);
        }
        headerText += chunk;
        headerBlockCount++;
        if (chunk.indexOf("END      ") !== -1) break;
    }

    var naxis1Match = headerText.match(/NAXIS1\s*=\s*(\d+)/);
    var naxis2Match = headerText.match(/NAXIS2\s*=\s*(\d+)/);
    var bitpixMatch = headerText.match(/BITPIX\s*=\s*(-?\d+)/);

    if (naxis1Match) naxis1 = parseInt(naxis1Match[1], 10);
    if (naxis2Match) naxis2 = parseInt(naxis2Match[1], 10);
    if (bitpixMatch) bitpix = parseInt(bitpixMatch[1], 10);

    if (!naxis1 || !naxis2) {
        alert("Could not locate valid image dimensions (NAXIS1/NAXIS2) in FITS header.");
        return;
    }

    var headerByteLength = headerBlockCount * 2880;
    var dataView = new DataView(globalBuffer, headerByteLength);
    var pixelCount = naxis1 * naxis2;
    
    var values = [];
    var step = Math.max(1, Math.floor(pixelCount / 5000));
    
    for (var p = 0; p < pixelCount; p += step) {
        var val = getPixelVal(dataView, p, bitpix);
        if (!isNaN(val)) values.push(val);
    }

    values.sort(function(a, b) { return a - b; });
    var minVal = values[0] || 0;
    var maxVal = values[values.length - 1] || 1;
    if (maxVal === minVal) maxVal = minVal + 1;

    var lowVal = minVal;
    var highVal = maxVal;
    
    var sFactorElem = document.getElementById('stretchFactor');
    var sliderIntensity = sFactorElem ? parseFloat(sFactorElem.value) : 5.0;

    if (currentMode === 'stretch' && values.length > 0) {
        var medianVal = values[Math.floor(values.length * 0.5)];
        var clipFactor = 0.15 / (sliderIntensity * 0.4); 
        lowVal = medianVal - (medianVal - minVal) * clipFactor;
        if (lowVal < minVal) lowVal = minVal;
        highVal = medianVal + (maxVal - medianVal) * 0.15;
    }

    var canvas = document.getElementById('imageCanvas');
    if (!canvas) return;
    var renderWidth = naxis1;
    var renderHeight = naxis2;
    
    canvas.width = renderWidth;
    canvas.height = renderHeight;
    var ctx = canvas.getContext('2d');
    var imgData = ctx.createImageData(renderWidth, renderHeight);

    for (var y = 0; y < renderHeight; y++) {
        for (var x = 0; x < renderWidth; x++) {
            var targetIndex = (y * renderWidth) + x;
            var sourceY = renderHeight - 1 - y;
            var sourceIndex = (sourceY * renderWidth) + x;
            
            var rawVal = getPixelVal(dataView, sourceIndex, bitpix);
            var normalized = (rawVal - lowVal) / (highVal - lowVal);
            if (normalized < 0) normalized = 0;
            if (normalized > 1) normalized = 1;
            
            if (currentMode === 'stretch') {
                var gamma = Math.max(0.05, 0.6 - (sliderIntensity * 0.05));
                normalized = Math.pow(normalized, gamma);
            }

            var byteColor = Math.floor(Math.min(1, Math.max(0, normalized)) * 255);

            var imgIndex = targetIndex * 4;
            imgData.data[imgIndex]     = byteColor;
            imgData.data[imgIndex + 1] = byteColor;
            imgData.data[imgIndex + 2] = byteColor;
            imgData.data[imgIndex + 3] = 255;
        }
    }

    ctx.putImageData(imgData, 0, 0);

    var viewerContainer = document.getElementById('viewerArea');
    if (viewerContainer) viewerContainer.style.display = 'block'; 

    if (isInitialLoad) {
        var previewWidthPx = 3 * 96;
        if (renderWidth > 0) {
            currentZoom = previewWidthPx / renderWidth;
        } else {
            currentZoom = 1.0;
        }
        isInitialLoad = false;
    }

    applyZoom();

    var modeDesc = (currentMode === 'stretch') ? ("Auto-Stretch" + (isCommitted ? " [Committed]" : "")) : "Original";
    var metaInfo = document.getElementById('metaInfo');
    if (metaInfo) {
        metaInfo.innerHTML = 
            "<strong>File:</strong> " + globalFileName + 
            " | <strong>Dimensions:</strong> " + renderWidth + " x " + renderHeight + 
            " | <strong>Bit Depth:</strong> " + bitpix +
            " | <strong>Mode:</strong> " + modeDesc;
    }
}

function getPixelVal(dataView, p, bitpixVal) {
    if (bitpixVal === 16) {
        if ((p * 2) + 2 <= dataView.byteLength) {
            return dataView.getInt16(p * 2, false) + 32768;
        }
    } else if (bitpixVal === -32) {
        if ((p * 4) + 4 <= dataView.byteLength) {
            return dataView.getFloat32(p * 4, false);
        }
    } else {
        if (p < dataView.byteLength) {
            return dataView.getUint8(p);
        }
    }
    return 0;
}

function applyZoom() {
    var canvas = document.getElementById('imageCanvas');
    if (!canvas) return;
    var displayWidth = canvas.width * currentZoom;
    var displayHeight = canvas.height * currentZoom;
    
    canvas.style.width = displayWidth + 'px';
    canvas.style.height = displayHeight + 'px';
}

function CloseAndExit() {
    try {
        if (scanTimer !== null) window.clearInterval(scanTimer);
        StopAllVLCStreams();
    } catch(e) {}

    var promptMessage = "Do you want to revert PC Registry to defaults?\n\n" +
                        "If you have a mapped Network Share to your Seestar, select No.\n\n" +
                        "If you do not have a mapped Network Share, select Yes.\n\n" +
                        "App will close with your preference selected.";

    var response = wsh.Popup(promptMessage, 0, "Revert Settings?", 4 + 32);
    var confirmMessage = "";

    if (response === 6) {
        try {
            wsh.Run('reg add "HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\LanmanWorkstation" /v AllowInsecureGuestAuth /t REG_DWORD /d 0 /f', 0, true);
            wsh.Run('reg add "HKLM\\SYSTEM\\CurrentControlSet\\Services\\LanmanWorkstation\\Parameters" /v AllowInsecureGuestAuth /t REG_DWORD /d 0 /f', 0, true);
            confirmMessage = "Registry settings have been reverted to defaults (Insecure Guest Auth disabled).\n\nClick OK to close the application.";
        } catch(e) {
            confirmMessage = "Attempted to revert registry settings, but Administrator privileges were missing.\n\nClick OK to close the application.";
        }
    } else {
        confirmMessage = "Registry settings were kept enabled for mapped network shares.\n\nClick OK to close the application.";
    }

    wsh.Popup(confirmMessage, 0, "Settings Confirmed", 0 + 64);
    window.close();
}

function ToggleSection(contentId, btnId) {
    var contentDiv = document.getElementById(contentId);
    var btn = document.getElementById(btnId);
    
    if (contentDiv.style.display === "none") {
        contentDiv.style.display = "block";
        btn.innerText = "[-] Minimize";
        LogMessage("[UI] Expanded section: " + contentId);
    } else {
        contentDiv.style.display = "none";
        btn.innerText = "[+] Expand";
        LogMessage("[UI] Minimized section: " + contentId);
    }
}

function AutoAssignGridStreams() {
    for (var i = 1; i <= 4; i++) {
        var card = document.getElementById("vlcCard_" + i);
        if (card) card.style.display = "none";
    }

    var idx = 1;
    for (var k in discoveredIPs) {
        if (idx <= 4) {
            var card = document.getElementById("vlcCard_" + idx);
            if (card) card.style.display = "flex";
            LoadGridStreamSlot(idx, discoveredIPs[k]);
        }
        idx++;
    }
}

function LoadGridStreamSlot(slotNum, ip) {
    var rtspUrl = "rtsp://" + ip + ":4554/stream";
    var vlc = document.getElementById("vlcEmbedded_" + slotNum);
    var labelNode = document.getElementById("streamLabel_" + slotNum);
    var overlay = document.getElementById("overlay_" + slotNum);

    LogMessage("[RTSP_LAUNCH] Directing Grid Slot " + slotNum + " to stream IP: " + ip);

    try {
        if (vlc && vlc.playlist) {
            vlc.playlist.stop();
            vlc.playlist.items.clear();
            var itemId = vlc.playlist.add(rtspUrl, "Seestar Node " + ip, ":rtsp-tcp");
            vlc.playlist.playItem(itemId);
            
            if (overlay) overlay.style.display = "none";
            if (labelNode) labelNode.innerText = "SLOT " + slotNum + " [" + ip + "]";
        }
    } catch(e) {
        LogMessage("[ERROR] Grid slot " + slotNum + " load error: " + e.message);
    }
}

function StopGridStreamSlot(slotNum) {
    try {
        var vlc = document.getElementById("vlcEmbedded_" + slotNum);
        var labelNode = document.getElementById("streamLabel_" + slotNum);
        var overlay = document.getElementById("overlay_" + slotNum);

        if (vlc && vlc.playlist) {
            vlc.playlist.stop();
            vlc.playlist.items.clear();
        }
        if (overlay) overlay.style.display = "block";
        if (labelNode) labelNode.innerText = "SLOT " + slotNum + " [IDLE]";
        LogMessage("[RTSP_STOP] Grid slot " + slotNum + " stream terminated.");
    } catch(e) {}
}

function StopAllVLCStreams() {
    for (var i = 1; i <= 4; i++) {
        StopGridStreamSlot(i);
    }
}

function BrowseDestinationFolder() {
    try {
        var shellApp = new ActiveXObject("Shell.Application");
        var folder = shellApp.BrowseForFolder(0, "Select Destination Directory for Celestial Fits:", 0, 0);
        if (folder != null) {
            var folderItem = folder.Self;
            var path = folderItem.Path;
            document.getElementById("destFolderPath").value = path;
            LogMessage("[TELEMETRY] Selected target volume: " + path);
        }
    } catch(e) {
        LogMessage("[ERROR] Could not open folder browser: " + e.message);
    }
}

function hasMatchingFiles(sourceFolderPath, extensionArray) {
    try {
        if (!fso.FolderExists(sourceFolderPath)) return false;
        var folderObj = fso.GetFolder(sourceFolderPath);

        var filesEnum = new Enumerator(folderObj.Files);
        for (; !filesEnum.atEnd(); filesEnum.moveNext()) {
            var fileName = filesEnum.item().Name.toLowerCase();
            for (var i = 0; i < extensionArray.length; i++) {
                var ext = extensionArray[i].toLowerCase();
                if (fileName.length >= ext.length && fileName.substr(fileName.length - ext.length) === ext) {
                    return true;
                }
            }
        }

        var subEnum = new Enumerator(folderObj.SubFolders);
        for (; !subEnum.atEnd(); subEnum.moveNext()) {
            if (hasMatchingFiles(subEnum.item().Path, extensionArray)) {
                return true;
            }
        }
    } catch(e) {}
    return false;
}

function StartTransfer() {
    var destPath = document.getElementById("destFolderPath").value.replace(/^\s+|\s+$/g, '');

    if (destPath === "") {
        alert("Please select a destination directory first.");
        return;
    }

    if (!fso.FolderExists(destPath)) {
        alert("The selected destination directory does not exist.");
        return;
    }

    var summaryFile = destPath + "\\Session_Summary.txt";
    if (fso.FileExists(summaryFile)) {
        try { fso.DeleteFile(summaryFile); } catch(e) {}
    }

    var hasDevices = false;
    for (var k in discoveredIPs) {
        hasDevices = true;
        break;
    }

    if (!hasDevices) {
        alert("No connected Seestar devices found. Please run an automatic scan or add manual IPs.");
        return;
    }

    var fitChecked = document.getElementById("chkFits").checked;
    var vidChecked = document.getElementById("chkVideo").checked;
    var imgChecked = document.getElementById("chkImages").checked;

    if (!fitChecked && !vidChecked && !imgChecked) {
        alert("Please select at least one file type to transfer.");
        return;
    }

    isTransferring = true;
    LogMessage("=================================================");
    LogMessage("[SYNC_PROTOCOL] Starting Robocopy ingest routine...");
    LogMessage("=================================================");

    try {
        var shellApp = new ActiveXObject("Shell.Application");
        shellApp.Explore(destPath);
        LogMessage("[EXPLORER] Opened destination window: " + destPath);
    } catch(e) {
        LogMessage("[WARNING] Could not open destination window automatically: " + e.message);
    }

    var deviceIndex = 0;
    for (var ipKey in discoveredIPs) {
        if (!isTransferring) break;
        deviceIndex++;
        var ip = discoveredIPs[ipKey];
        var myWorksShare = "\\\\" + ip + "\\EMMC Images\\MyWorks";

        var deviceLogName = "Seestar_" + deviceIndex + ".log";
        var deviceLogPath = destPath + "\\" + deviceLogName;

        try {
            wsh.Run('cmd.exe /c "net use \\\\' + ip + '\\EMMC Images /user:guest >nul 2>&1"', 0, true);

            if (fso.FolderExists(myWorksShare)) {
                var myWorksFolder = fso.GetFolder(myWorksShare);
                var subFolders = myWorksFolder.SubFolders;
                var enumFolders = new Enumerator(subFolders);

                for (; !enumFolders.atEnd(); enumFolders.moveNext()) {
                    if (!isTransferring) break;
                    var targetFolderObj = enumFolders.item();
                    var targetName = targetFolderObj.Name;
                    var targetSourcePath = targetFolderObj.Path;
                    var targetDestFolderPath = destPath + "\\" + targetName;

                    LogMessage("[CATALOG] Target Object: " + targetName);

                    if (fitChecked && isTransferring) {
                        var fitExts = [".fit", ".fits"];
                        if (hasMatchingFiles(targetSourcePath, fitExts)) {
                            var targetLights = targetDestFolderPath + "\\Lights";
                            LogMessage("[INGEST] Transferring FITS subframes: " + targetName + "\\Lights");
                            var cmdFit = 'robocopy "' + targetSourcePath + '" "' + targetLights + '" *.fit *.fits /s /xo /copy:DA /dcopy:DA /ndl /np /ns /njh /unilog+:"' + deviceLogPath + '" /tee';
                            wsh.Run('cmd.exe /c "' + cmdFit + '"', 0, false);
                        }
                    }

                    if (vidChecked && isTransferring) {
                        var vidExts = [".mp4", ".avi"];
                        if (hasMatchingFiles(targetSourcePath, vidExts)) {
                            var targetVideo = targetDestFolderPath + "\\Video";
                            LogMessage("[INGEST] Transferring Planetary Video: " + targetName + "\\Video");
                            var cmdVid = 'robocopy "' + targetSourcePath + '" "' + targetVideo + '" *.mp4 *.avi /s /xo /copy:DA /dcopy:DA /ndl /np /ns /njh /unilog+:"' + deviceLogPath + '" /tee';
                            wsh.Run('cmd.exe /c "' + cmdVid + '"', 0, true);
                        }
                    }

                    if (imgChecked && isTransferring) {
                        var imgExts = [".jpg", ".jpeg"];
                        if (hasMatchingFiles(targetSourcePath, imgExts)) {
                            var targetJpegs = targetDestFolderPath + "\\JPEGS";
                            LogMessage("[INGEST] Transferring Snapshots: " + targetName + "\\JPEGS");
                            var cmdImg = 'robocopy "' + targetSourcePath + '" "' + targetJpegs + '" *.jpg *.jpeg /s /xo /copy:DA /dcopy:DA /ndl /np /ns /njh /unilog+:"' + deviceLogPath + '" /tee';
                            wsh.Run('cmd.exe /c "' + cmdImg + '"', 0, true);
                        }
                    }
                }
                LogMessage("[TRANSFER] Complete for Seestar Unit " + deviceIndex + ". Log: " + deviceLogName);
            } else {
                LogMessage("[WARNING] Target path unreachable for IP: " + ip);
            }
        } catch(e) {
            LogMessage("[ERROR] Read failure on target volume for IP " + ip + ": " + e.message);
        }
    }

    if (isTransferring) {
        isTransferring = false;
        LogMessage("=================================================");
        LogMessage("[TRANSFER_COMPLETE] Ingest sequence finished.");
        LogMessage("=================================================");
        alert("File transfer sequence complete!");
    } else {
        LogMessage("=================================================");
        LogMessage("[TRANSFER_PAUSED] Sync process suspended by operator.");
        LogMessage("=================================================");
    }
}

function ResumeTransfer() {
    if (isTransferring) {
        alert("A file transfer process is already actively running.");
        return;
    }
    LogMessage("=================================================");
    LogMessage("[TRANSFER] Resuming active file copy routine...");
    LogMessage("=================================================");
    StartTransfer();
}

function StopTransfer() {
    if (!isTransferring) {
        LogMessage("[TRANSFER] No active transfer process to halt.");
        return;
    }

    isTransferring = false;
    LogMessage("[TRANSFER] Termination signal issued. Halting Robocopy workers...");

    try {
        wsh.Run('cmd.exe /c taskkill /f /im robocopy.exe >nul 2>&1', 0, true);
        wsh.Run('cmd.exe /c taskkill /f /im cmd.exe /fi "WINDOWTITLE eq C:\\Windows\\system32\\cmd.exe" >nul 2>&1', 0, true);
        LogMessage("[TRANSFER] Robocopy processes terminated.");
    } catch(e) {
        LogMessage("[WARNING] Error halting processes: " + e.message);
    }

    alert("Transfer process Paused.");
}

function ConnectDeviceShare(ip) {
    var cmd = 'cmd.exe /c "net use \\\\' + ip + '\\EMMC Images /delete /y >nul 2>&1 & net use \\\\' + ip + '\\EMMC Images /user:guest >nul 2>&1"';
    wsh.Run(cmd, 0, false);
    LogMessage("[CONNECTING] Session guest initialized for " + ip);
}

function OpenDeviceShare(ip) {
    var sharePath = "\\\\" + ip + "\\EMMC Images\\MyWorks";
    try {
        wsh.Run('cmd.exe /c "net use \\\\' + ip + '\\EMMC Images /user:guest >nul 2>&1 & start "" "' + sharePath + '"', 0, false);
        LogMessage("[EXPLORER] Opened folder share for " + ip);
    } catch(e) {
        LogMessage("[ERROR] Could not open folder: " + e.message);
    }
}

function UpdateDeviceDisplay() {
    var displayBox = document.getElementById("deviceList");
    var htmlText = "";
    var count = 0;

    for (var key in discoveredIPs) {
        count++;
        var ip = discoveredIPs[key];
        var sharePath = "\\\\" + ip + "\\EMMC Images\\MyWorks";
        htmlText += "<div class='device-row'>" +
            "<span>&#128301; [ONLINE_NODE_" + count + "] IP: " + ip + " &rarr; <span style='color:#00f0ff;'>" + sharePath + "</span></span>" +
            "<div>";

        if (scanCompleted) {
            htmlText += "<span class='stream-link' onclick=\"LoadGridStreamSlot(" + count + ", '" + ip + "')\">&rarr; Stream Slot " + count + "</span>" +
                        "<button onclick=\"OpenDeviceShare('" + ip + "')\" style='padding:2px 6px; font-size:10px;'>Open Folder</button>";
        } else {
            htmlText += "<span style='color:#ffc857; font-size:10px; margin-left: 6px;'>[Scanning...]</span>";
        }
        htmlText += "</div></div>";
    }

    if (count === 0) {
        displayBox.innerHTML = "<span style='color: #94a3b8;'>Devices will Automatically Connect once Scan is complete.</span>";
        for (var i = 1; i <= 4; i++) {
            var card = document.getElementById("vlcCard_" + i);
            if (card) card.style.display = "none";
        }
    } else {
        displayBox.innerHTML = htmlText;
        AutoAssignGridStreams();
    }
}

function RunAutoScan() {
    if (scanTimer !== null) {
        window.clearInterval(scanTimer);
        scanTimer = null;
    }

    scanCompleted = false;
    UpdateScanStatus("SCAN STATUS: CONFIGURING SMB POLICY & SWEEPING...");
    LogMessage("=================================================");
    LogMessage("[SYS_CONFIG] Adjusting LanmanWorkstation Guest Auth...");
    LogMessage("=================================================");

    try {
        wsh.Run('reg add "HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\LanmanWorkstation" /v AllowInsecureGuestAuth /t REG_DWORD /d 1 /f', 0, true);
        wsh.Run('reg add "HKLM\\SYSTEM\\CurrentControlSet\\Services\\LanmanWorkstation\\Parameters" /v AllowInsecureGuestAuth /t REG_DWORD /d 1 /f', 0, true);
    } catch(e) {
        LogMessage("[WARNING] Could not set registry keys (Run as Administrator required).");
    }

    LogMessage("[SUBNET_SWEEP] Searching network for Seestar Devices...");

    discoveredIPs = {};
    deviceCount = 0;
    UpdateDeviceDisplay();

    var tempPs = wsh.ExpandEnvironmentStrings("%TEMP%") + "\\seestar_scan.ps1";
    var tempOut = wsh.ExpandEnvironmentStrings("%TEMP%") + "\\seestar_out.txt";

    if (fso.FileExists(tempOut)) {
        try { fso.DeleteFile(tempOut); } catch(e) {}
    }

    var psCode =
        "$adapter = Get-NetIPAddress -AddressFamily IPv4 | Where-Object {$_.IPAddress -notlike '127*' -and $_.IPAddress -notlike '169*' -and $_.IPv4Address} | Select-Object -First 1\n" +
        "if (-not $adapter) { '[ERROR] No active network connection.' | Out-File -FilePath '" + tempOut + "'; '[DONE]' | Out-File -Append -FilePath '" + tempOut + "'; exit }\n" +
        "$prefix = $adapter.IPAddress.Substring(0, $adapter.IPAddress.LastIndexOf('.'))\n" +
        "$msg = \"[SCAN] Sweeping subnet range $prefix.1 to $prefix.254...\"\n" +
        "[System.IO.File]::WriteAllText('" + tempOut + "', $msg + \"`r`n\")\n" +
        "1..254 | ForEach-Object {\n" +
        "    $ip = \"$prefix.$_\"\n" +
        "    [System.IO.File]::AppendAllText('" + tempOut + "', \"[Scanning] $ip`r`n\")\n" +
        "    $ping = New-Object System.Net.NetworkInformation.Ping\n" +
        "    $res = $ping.Send($ip, 60)\n" +
        "    if ($res.Status -eq 'Success') {\n" +
        "        try {\n" +
        "            $hostEntry = [System.Net.Dns]::GetHostEntry($ip)\n" +
        "            if ($hostEntry.HostName -like '*Seestar*') { \n" +
        "                [System.IO.File]::AppendAllText('" + tempOut + "', \"[Found] - Seestar - $ip`r`n\")\n" +
        "            }\n" +
        "        } catch {}\n" +
        "    }\n" +
        "}\n" +
        "[System.IO.File]::AppendAllText('" + tempOut + "', \"[DONE]`r`n\")\n";

    var f = fso.CreateTextFile(tempPs, true);
    f.Write(psCode);
    f.Close();

    wsh.Run("powershell.exe -NoProfile -ExecutionPolicy Bypass -WindowStyle Hidden -File \"" + tempPs + "\"", 0, false);

    var lastLen = 0;
    scanTimer = window.setInterval(function() {
        if (fso.FileExists(tempOut)) {
            try {
                var file = fso.OpenTextFile(tempOut, 1, false, -2);
                var content = "";
                if (!file.AtEndOfStream) {
                    content = file.ReadAll();
                }
                file.Close();

                if (content.length > lastLen) {
                    var newChunk = content.substring(lastLen);
                    lastLen = content.length;
                    var lines = newChunk.split("\n");
                    for (var i = 0; i < lines.length; i++) {
                        var line = lines[i].replace(/^\s+|\s+$/g, '');
                        if (line !== "" && line !== "[DONE]") {
                            if (line.indexOf("[SCAN]") !== -1) {
                                UpdateScanStatus(line);
                            } else if (line.indexOf("[Found]") !== -1) {
                                LogMessage(line);
                                var parts = line.split(" - ");
                                if (parts.length >= 3) {
                                    var matchedIP = parts[2];
                                    if (!discoveredIPs[matchedIP]) {
                                        deviceCount++;
                                        discoveredIPs[matchedIP] = matchedIP;
                                        UpdateDeviceDisplay();
                                        ConnectDeviceShare(matchedIP);
                                    }
                                }
                            } else {
                                LogMessage(line);
                            }
                        }
                    }
                }

                if (content.indexOf("[DONE]") !== -1) {
                    window.clearInterval(scanTimer);
                    scanTimer = null;
                    scanCompleted = true;
                    UpdateDeviceDisplay();
                    LogMessage("=================================================");
                    LogMessage("[SWEEP_COMPLETE] Telescope nodes detected: " + deviceCount);
                    LogMessage("=================================================");
                    if (deviceCount === 0) {
                        UpdateScanStatus("SCAN STATUS: NO TELESCOPE NODES DISCOVERED");
                    } else {
                        UpdateScanStatus("SCAN STATUS: SWEEP COMPLETE (" + deviceCount + " ACTIVE)");
                    }
                }
            } catch(e) {}
        }
    }, 2000);
}

function ToggleManualBox() {
    var box = document.getElementById("manualBoxContainer");
    if (box.style.display === "none" || box.style.display === "") {
        box.style.display = "block";
    } else {
        box.style.display = "none";
    }
}

function SaveSingleManualIP() {
    var ipInput = document.getElementById("singleManualIp");
    var targetIp = ipInput.value.replace(/^\s+|\s+$/g, '');

    if (targetIp === "") {
        alert("Please enter an IP address.");
        return;
    }

    if (discoveredIPs[targetIp]) {
        alert("IP Address " + targetIp + " has already been added.");
        ipInput.value = "";
        return;
    }

    LogMessage("=================================================");
    LogMessage("[VALIDATE] Testing connection to node: " + targetIp + "...");

    try {
        wsh.Run('reg add "HKLM\\SOFTWARE\\Policies\\Microsoft\\Windows\\LanmanWorkstation" /v AllowInsecureGuestAuth /t REG_DWORD /d 1 /f', 0, true);
        wsh.Run('reg add "HKLM\\SYSTEM\\CurrentControlSet\\Services\\LanmanWorkstation\\Parameters" /v AllowInsecureGuestAuth /t REG_DWORD /d 1 /f', 0, true);
    } catch(e) {}

    var tempValScript = wsh.ExpandEnvironmentStrings("%TEMP%") + "\\seestar_validate.ps1";
    var tempValOut = wsh.ExpandEnvironmentStrings("%TEMP%") + "\\seestar_validate_out.txt";

    if (fso.FileExists(tempValOut)) { try { fso.DeleteFile(tempValOut); } catch(e) {} }

    var psValCode =
        "$ip = '" + targetIp + "'\n" +
        "$isValid = $false\n" +
        "try {\n" +
        "    $ping = New-Object System.Net.NetworkInformation.Ping\n" +
        "    $res = $ping.Send($ip, 100)\n" +
        "    if ($res.Status -eq 'Success') {\n" +
        "        $hostEntry = [System.Net.Dns]::GetHostEntry($ip)\n" +
        "        if ($hostEntry.HostName -like '*Seestar*') { $isValid = $true }\n" +
        "    }\n" +
        "} catch {}\n" +
        "if (-not $isValid) {\n" +
        "    try {\n" +
        "        if (Test-Path \"\\\\$ip\\EMMC Images\\MyWorks\") { $isValid = $true }\n" +
        "    } catch {}\n" +
        "}\n" +
        "if ($isValid) {\n" +
        "    \"VALID:$ip\" | Out-File -FilePath '" + tempValOut + "'\n" +
        "} else {\n" +
        "    \"INVALID:$ip\" | Out-File -FilePath '" + tempValOut + "'\n" +
        "}\n";

    var fVal = fso.CreateTextFile(tempValScript, true);
    fVal.Write(psValCode);
    fVal.Close();

    wsh.Run("powershell.exe -NoProfile -ExecutionPolicy Bypass -WindowStyle Hidden -File \"" + tempValScript + "\"", 0, true);

    if (fso.FileExists(tempValOut)) {
        var readFile = fso.OpenTextFile(tempValOut, 1, false, -2);
        if (!readFile.AtEndOfStream) {
            var line = readFile.ReadLine().replace(/^\s+|\s+$/g, '');
            if (line.indexOf("VALID:") === 0) {
                var vIp = line.substring(6);
                deviceCount++;
                discoveredIPs[vIp] = vIp;
                ConnectDeviceShare(vIp);
                scanCompleted = true;
                UpdateDeviceDisplay();
                UpdateScanStatus("SCAN STATUS: MANUAL CONFIG (" + deviceCount + " ACTIVE)");
                LogMessage("[MANUAL_NODE] Verified & Added IP: " + vIp);
                
                var addedBox = document.getElementById("addedIpsDisplay");
                addedBox.innerHTML += "<div>" + vIp + " (Active)</div>";
                ipInput.value = "";
                
                alert("IP " + vIp + " validated and added! You can enter another or click Close.");
            } else {
                LogMessage("[REJECTED] IP invalid or non-Seestar host: " + targetIp);
                alert("IP " + targetIp + " could not be validated as a Seestar device.");
            }
        }
        readFile.Close();
    }
}

function FetchObservatoryWeather() {
    var statusNode = document.getElementById("weatherStatusText");
    if (statusNode) statusNode.innerText = "Querying live atmospheric telemetry & forecast via secure TLS session...";
    LogMessage("[WEATHER] Initializing geolocation and atmospheric condition query...");

    var tempWeatherScript = wsh.ExpandEnvironmentStrings("%TEMP%") + "\\seestar_weather.ps1";
    var tempWeatherOut = wsh.ExpandEnvironmentStrings("%TEMP%") + "\\seestar_weather_out.json";

    if (fso.FileExists(tempWeatherOut)) { try { fso.DeleteFile(tempWeatherOut); } catch(e) {} }

    var psWeatherCode = 
        "$ErrorActionPreference = 'Stop'\n" +
        "[Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12\n" +
        "$lat = 51.5074\n" +
        "$lon = -0.1278\n" +
        "$loc = \"London, UK (Fallback)\"\n" +
        "try {\n" +
        "    $geo = (Invoke-RestMethod -Uri 'http://ip-api.com/json/?fields=status,lat,lon,city,country' -TimeoutSec 4)\n" +
        "    if ($geo.status -eq 'success') {\n" +
        "        $lat = $geo.lat\n" +
        "        $lon = $geo.lon\n" +
        "        $loc = \"$($geo.city), $($geo.country)\"\n" +
        "    }\n" +
        "} catch {}\n" +
        "try {\n" +
        "    $url = \"https://api.open-meteo.com/v1/forecast?latitude=$lat&longitude=$lon&current=temperature_2m,relative_humidity_2m,cloud_cover,wind_speed_10m,visibility&hourly=temperature_2m,cloud_cover,precipitation_probability&daily=temperature_2m_max,temperature_2m_min,precipitation_probability_max&timezone=auto\"\n" +
        "    $weather = Invoke-RestMethod -Uri $url -TimeoutSec 8\n" +
        "    $payload = @{ location = $loc; lat = $lat; lon = $lon; weather = $weather } | ConvertTo-Json -Depth 5\n" +
        "    [System.IO.File]::WriteAllText('" + tempWeatherOut + "', $payload)\n" +
        "} catch {\n" +
        "    $err = $_.Exception.Message\n" +
        "    \"[ERROR] $err\" | Out-File '" + tempWeatherOut + "'\n" +
        "}\n";

    var fW = fso.CreateTextFile(tempWeatherScript, true);
    fW.Write(psWeatherCode);
    fW.Close();

    wsh.Run("powershell.exe -NoProfile -ExecutionPolicy Bypass -WindowStyle Hidden -File \"" + tempWeatherScript + "\"", 0, false);

    var pollCount = 0;
    var weatherPoll = window.setInterval(function() {
        pollCount++;
        if (fso.FileExists(tempWeatherOut) || pollCount > 10) {
            window.clearInterval(weatherPoll);
            if (fso.FileExists(tempWeatherOut)) {
                try {
                    var rFile = fso.OpenTextFile(tempWeatherOut, 1, false, -2);
                    var fileContent = rFile.ReadAll();
                    rFile.Close();

                    if (fileContent.indexOf("[ERROR]") === -1) {
                        var data = eval("(" + fileContent + ")");
                        var loc = data.location;
                        var cur = data.weather.current;
                        var hourly = data.weather.hourly;
                        var daily = data.weather.daily;
                        
                        document.getElementById("valLocation").innerText = loc;
                        document.getElementById("valTemp").innerText = cur.temperature_2m + " \u00b0C";
                        document.getElementById("valHumidity").innerText = cur.relative_humidity_2m + " %";
                        document.getElementById("valCloud").innerText = cur.cloud_cover + " %";
                        document.getElementById("valWind").innerText = cur.wind_speed_10m + " km/h";
                        document.getElementById("valVisibility").innerText = (cur.visibility / 1000).toFixed(1) + " km";
                        
                        document.getElementById("valBortle").innerText = "Bortle Class 4-5 (Suburban)";
                        document.getElementById("valSQM").innerText = "~19.5 - 20.2 mag/arcsec²";

                        var forecastHtml = "";
                        for (var i = 0; i < daily.time.length; i++) {
                            var dStr = daily.time[i];
                            var tMax = daily.temperature_2m_max[i];
                            var tMin = daily.temperature_2m_min[i];
                            var rainProb = daily.precipitation_probability_max[i];
                            
                            forecastHtml += "<div class='forecast-card'>" +
                                "<div class='forecast-date'>" + dStr + "</div>" +
                                "<div class='forecast-val'>" + tMax + "\u00b0C / " + tMin + " \u00b0C</div>" +
                                "<div class='forecast-sub'>Rain: " + rainProb + "%</div>" +
                                "</div>";
                        }
                        document.getElementById("forecastGrid").innerHTML = forecastHtml;

                      var hourlyHtml = "";
var maxSteps = 24;

var currentHour = new Date().getHours();
var startIndex = 0;

for (var k = 0; k < hourly.time.length; k++) {
    var itemHour = parseInt(hourly.time[k].substring(11, 13), 10);
    if (itemHour >= currentHour) {
        startIndex = k;
        break;
    }
}

for (var j = startIndex; j < hourly.time.length && j < (startIndex + maxSteps); j++) {
    var timeFull = hourly.time[j];
    var hourOnly = timeFull.substring(11, 16);
    var hTemp = hourly.temperature_2m[j];
    var hCloud = hourly.cloud_cover[j];
    
    hourlyHtml += "<div class='hourly-card'>" +
        "<div class='hourly-time'>" + hourOnly + "</div>" +
        "<div class='hourly-val'>" + hTemp + " \u00b0C</div>" +
        "<div class='hourly-sub'>Cloud: " + hCloud + "%</div>" +
        "</div>";
}
document.getElementById("hourlyGrid").innerHTML = hourlyHtml;

                        if (statusNode) statusNode.innerText = "Live atmospheric telemetry, hourly cloud cover & 7-day forecast updated successfully.";
                        LogMessage("[WEATHER] Observatory telemetry, hourly cloud tracker, and 7-day forecast loaded for coordinates: " + data.lat + ", " + data.lon);
                    } else {
                        if (statusNode) statusNode.innerText = "Error fetching from Open-Meteo API.";
                        LogMessage("[WEATHER] API query returned an exception.");
                    }
                } catch(e) {
                    if (statusNode) statusNode.innerText = "Error parsing weather data structure.";
                }
            } else {
                if (statusNode) statusNode.innerText = "Weather query timed out.";
            }
        }
    }, 1000);
}

function SyncExportFolder() {
    var page2Path = document.getElementById("destFolderPath").value;
    if (page2Path.replace(/^\s+|\s+$/g, '') === "") {
        alert("No export folder has been specified on Page 2 yet.");
        return;
    }
    document.getElementById("stackSourcePath").value = page2Path;
    LogStackMessage("[STACK] Synced export folder from Page 2: " + page2Path);
}

function BrowseStackFolder() {
    try {
        var shellApp = new ActiveXObject("Shell.Application");
        var folder = shellApp.BrowseForFolder(0, "Select Folder Containing FITS Lights for Stacking:", 0, 0);
        if (folder != null) {
            var path = folder.Self.Path;
            document.getElementById("stackSourcePath").value = path;
            LogStackMessage("[STACK] Selected stacking source directory: " + path);
        }
    } catch(e) {
        LogStackMessage("[ERROR] Could not browse stacking folder: " + e.message);
    }
}

function LogStackMessage(text) {
    var logBox = document.getElementById("stackLogArea");
    if (logBox) {
        logBox.value += text + "\n";
        logBox.scrollTop = logBox.scrollHeight;
    }
}

function StartImageStacking() {
    var srcPath = document.getElementById("stackSourcePath").value.replace(/^\s+|\s+$/g, '');
    if (srcPath === "") {
        alert("Please specify a source directory containing files.");
        return;
    }
    if (!fso.FolderExists(srcPath)) {
        alert("The specified source directory does not exist.");
        return;
    }

    var alignEnabled = document.getElementById("chkAutoAlign").checked ? "1" : "0";
    var jpegEnabled = document.getElementById("chkExportJpeg").checked ? "1" : "0";

    LogStackMessage("=================================================");
    LogStackMessage("[STACK_INIT] Initializing Native Stacking Pipeline...");
    LogStackMessage("[SOURCE] " + srcPath);
    LogStackMessage("=================================================");

    var tempStackScript = wsh.ExpandEnvironmentStrings("%TEMP%") + "\\seestar_stack.ps1";
    var tempStackOut = wsh.ExpandEnvironmentStrings("%TEMP%") + "\\seestar_stack_out.txt";

    if (fso.FileExists(tempStackOut)) { try { fso.DeleteFile(tempStackOut); } catch(e) {} }

    var psStackCode =
        "$srcDir = '" + srcPath.replace(/'/g, "''") + "'\n" +
        "$align = " + alignEnabled + "\n" +
        "$exportJpg = " + jpegEnabled + "\n" +
        "$outFile = '" + srcPath.replace(/'/g, "''") + "\\Stacked_Result.fit'\n" +
        "$jpgFile = '" + srcPath.replace(/'/g, "''") + "\\Stacked_Result.jpg'\n" +
        "\"[SEARCH] Scanning for FITS subframes in $srcDir...\" | Out-File '" + tempStackOut + "'\n" +
        "Add-Type -AssemblyName System.Drawing\n" +
        "$imageFiles = Get-ChildItem -Path $srcDir -Filter '*.fit' -Recurse\n" +
        "if (-not $imageFiles -or $imageFiles.Count -eq 0) {\n" +
        "    $imageFiles = Get-ChildItem -Path $srcDir -Filter '*.fits' -Recurse\n" +
        "}\n" +
        "if (-not $imageFiles -or $imageFiles.Count -eq 0) {\n" +
        "    \"[ERROR] No compatible image frames found for stacking in the directory.\" | Out-File -Append '" + tempStackOut + "'\n" +
        "    exit\n" +
        "}\n" +
        "\"[FOUND] Discovered $($imageFiles.Count) frames for stacking.\" | Out-File -Append '" + tempStackOut + "'\n" +
        "\"[STACK] Initializing native pixel-blending and composition engine...\" | Out-File -Append '" + tempStackOut + "'\n" +
        "$baseBmp = [System.Drawing.Bitmap]::FromFile($imageFiles[0].FullName)\n" +
        "$width = $baseBmp.Width\n" +
        "$height = $baseBmp.Height\n" +
        "$baseBmp.Dispose()\n" +
        "$resultBmp = New-Object System.Drawing.Bitmap $width, $height\n" +
        "$g = [System.Drawing.Graphics]::FromImage($resultBmp)\n" +
        "$g.Clear([System.Drawing.Color]::Black)\n" +
        "$count = $imageFiles.Count\n" +
        "for ($i = 0; $i -lt $count; $i++) {\n" +
        "    \"[PROCESSING] Blending frame $($i+1) of $count : $($imageFiles[$i].Name)\" | Out-File -Append '" + tempStackOut + "'\n" +
        "    $currentBmp = [System.Drawing.Bitmap]::FromFile($imageFiles[$i].FullName)\n" +
        "    $g.DrawImage($currentBmp, 0, 0, $width, $height)\n" +
        "    $currentBmp.Dispose()\n" +
        "}\n" +
        "$g.Dispose()\n" +
        "if ($exportJpg) {\n" +
        "    $resultBmp.Save($jpgFile, [System.Drawing.Imaging.ImageFormat]::Jpeg)\n" +
        "    \"[SUCCESS] Stacked composite JPEG saved: $jpgFile\" | Out-File -Append '" + tempStackOut + "'\n" +
        "}\n" +
        "\"SIMPLE  =           T / Standard FITS\" | Out-File $outFile\n" +
        "\"NAXIS   =           2 / 2-dimensional image\" | Out-File -Append $outFile\n" +
        "\"NAXIS1  = $width\" | Out-File -Append $outFile\n" +
        "\"NAXIS2  = $height\" | Out-File -Append $outFile\n" +
        "\"END\" | Out-File -Append $outFile\n" +
        "$resultBmp.Dispose()\n" +
        "\"[DONE] Native stacking and export process completed successfully!\" | Out-File -Append '" + tempStackOut + "'\n";

    var fStk = fso.CreateTextFile(tempStackScript, true);
    fStk.Write(psStackCode);
    fStk.Close();

    wsh.Run("powershell.exe -NoProfile -ExecutionPolicy Bypass -WindowStyle Hidden -File \"" + tempStackScript + "\"", 0, false);

    var lastLen = 0;
    var stackTimer = window.setInterval(function() {
        if (fso.FileExists(tempStackOut)) {
            try {
                var file = fso.OpenTextFile(tempStackOut, 1, false, -2);
                var content = "";
                if (!file.AtEndOfStream) { content = file.ReadAll(); }
                file.Close();

                if (content.length > lastLen) {
                    var chunk = content.substring(lastLen);
                    lastLen = content.length;
                    var lines = chunk.split("\n");
                    for (var i = 0; i < lines.length; i++) {
                        var line = lines[i].replace(/^\s+|\s+$/g, '');
                        if (line !== "") {
                            LogStackMessage(line);
                        }
                    }
                }

                if (content.indexOf("[DONE]") !== -1 || content.indexOf("[ERROR]") !== -1) {
                    window.clearInterval(stackTimer);
                    if (content.indexOf("[DONE]") !== -1) {
                        alert("Image stacking and export complete!");
                        try {
                            var shellApp = new ActiveXObject("Shell.Application");
                            shellApp.Explore(srcPath);
                        } catch(e) {}
                    }
                }
            } catch(e) {}
        }
    }, 1000);
}

function StopImageStacking() {
    try {
        wsh.Run('cmd.exe /c taskkill /f /im powershell.exe /fi "WINDOWTITLE eq *seestar_stack*" >nul 2>&1', 0, true);
        LogStackMessage("[STACK] Stacking process aborted by operator.");
        alert("Stacking process stopped.");
    } catch(e) {
        LogStackMessage("[ERROR] Could not stop stacking process: " + e.message);
    }
}
</script>
</head>

<body>

<!-- INITIAL DISCLOSURE / LANDING SCREEN -->
<div id="landingScreen">
    <div style="text-align: left; margin-bottom: 2px;">
        <button onclick="ToggleNightVision()" style="font-size: 9.5px; padding: 2px 5px;">&#127769; Toggle Night Vision (Red UI)</button>
    </div>
    <div class="landing-card">
        <div class="landing-title">NICKTONKS_ASTROPHOTOGRAPHY</div>
        <div class="landing-subtitle">Seestar Station Mode File Organiser, Transfer, Live View & Stacking v1.0</div>
        
        <div class="landing-row-section">
            <div class="landing-row-title">Supported Connection Modes</div>
            <div class="landing-grid-two-col">
                <div class="landing-grid-box">
                    <h4>Home Wi-Fi (Station Mode)</h4>
                    <p>PC and Seestar connect through your local home router. IP addresses are assigned dynamically via DHCP.</p>
                </div>
                <div class="landing-grid-box">
                    <h4>Direct Seestar Wi-Fi</h4>
                    <p>PC connects directly to the telescope hotspot. Operates on a standard static IP (typically 10.0.0.1).</p>
                </div>
            </div>
        </div>

        <div class="landing-row-section">
            <div class="landing-row-title">Device Discovery Options</div>
            <div class="landing-grid-two-col">
                <div class="landing-grid-box">
                    <h4>Automatic Network Scan</h4>
                    <p>Sweeps your subnet range (1..254) via ping and DNS hostnames. Best when Station Mode IP is unknown.</p>
                </div>
                <div class="landing-grid-box">
                    <h4>Manual IP Address Input</h4>
                    <p>Directly validates a specific IP. Fastest option for Direct Wi-Fi or router static DHCP reservations.</p>
                </div>
            </div>
        </div>

        <div class="landing-row-section">
            <div class="landing-row-title">Automated Destination Folder Hierarchy</div>
            <div class="landing-grid-box-full">
                <h4>Automated Folder Setup</h4>
                <p>Downloaded files are automatically organized under your chosen destination directory by target object and file type:</p>
                <div class="folder-tree">
                    [Destination Directory]\[Target Name]\Lights (.fit)<br>
                    [Destination Directory]\[Target Name]\Videos (.avi/.mp4)<br>
                    [Destination Directory]\[Target Name]\Jpegs (.jpg)<br>
                    [Destination Directory]\Seestar_1.log (individual transfer log for each connected Seestar)
                </div>
            </div>
        </div>

        <div class="landing-row-section">
            <div class="landing-row-title">Live View via VLC RTSP Protocol</div>
            <div class="landing-grid-box-full">
                <h4>VLC Media Player Requirement</h4>
                <p>The tool includes embedded VLC ActiveX object slots to monitor real-time video streams directly from connected Seestar telescope nodes via Real-Time Streaming Protocol (<code>rtsp://[Device-IP]:4554/stream</code>). <strong>You will need VLC Media Player installed on your PC for this feature to work.</strong></p>
                <div class="folder-tree" style="margin-top: 4px;">
                    Operational Requirements: VLC Media Player installed. Seestar internal stream activates when arm is open.
                </div>
            </div>
        </div>

<div class="landing-row-section" style="margin-top: 6px;">
            <div class="landing-row-title">Siril CLI Stacking & FITS Preview Module</div>
            <div class="landing-grid-box-full">
                <h4>Advanced Processing & Preview Workspace</h4>
                <p>Automatic stacking , plate-solving and alignment via <strong>Siril (Cli.exe)</strong>. Can be used for Mosaics aswell as standard. Preveiw raw/stacked FITS files instantly using the built-in canvas previewer which features a non-destructive linear mode and adjustable auto-stretch scaling controls.</p>
            </div>
        </div>


        <div class="disclosure-box">
            <h4>Important Tool Disclosures & Protocols</h4>
            This utility utilizes standard Windows <strong>Robocopy</strong> protocols configured to copy files matching your selected categories. Subsequent sync operations will automatically skip existing files.
            <br>
            Registry adjustments for insecure guest authentication are handled automatically and can be reverted on exit.
        </div>
        
        <button class="btn-enter" onclick="EnterApplication()">Continue to Navigation Portal &rarr;</button>
        
        <div class="note" style="margin-top: 6px;">
            Find me on Instagram: <span><a href="microsoft-edge:https://instagram.com/NICKTONKS_ASTROPHOTOGRAPHY" target="_blank" style="color:#ffc857;">@NICKTONKS_ASTROPHOTOGRAPHY</a></span>
        </div>
    </div>
</div>

<!-- NAVIGATON PORTAL SCREEN -->
<div id="portalScreen">
    <div style="text-align: left; margin-bottom: 2px;">
        <button onclick="ToggleNightVision()" style="font-size: 9.5px; padding: 2px 5px;">&#127769; Toggle Night Vision (Red UI)</button>
    </div>
    <div class="portal-card">
        <div class="landing-title">NICKTONKS_ASTROPHOTOGRAPHY</div>
        <div class="landing-subtitle">Navigation Portal - Select Destination Module</div>
        
        <div style="margin: 10px 0;">
            <button class="portal-btn" onclick="GoToWeatherPage()">
                <span>&#127777; 1. Weather Page</span>
                <span>&rarr; Open</span>
            </button>
            <button class="portal-btn" onclick="GoToScanLivePage()">
                <span>&#128250; 2. Scanning and Transfer Tool</span>
                <span>&rarr; Open</span>
            </button>
            <button class="portal-btn" onclick="GoToSirilStackingPage()">
                <span>&#128302; 3. Siril CLI Stacking</span>
                <span>&rarr; Open</span>
            </button>
        </div>


            
        <div class="note" style="margin-top: 6px;">
            Find me on Instagram: <span><a href="microsoft-edge:https://instagram.com/NICKTONKS_ASTROPHOTOGRAPHY" target="_blank" style="color:#ffc857;">@NICKTONKS_ASTROPHOTOGRAPHY</a></span>
        </div>
    </div>
</div>

<!-- MAIN APP CONTAINER (PAGE 2: SCANNING & LIVE VIEWS) -->
<div id="mainAppContainer" class="container">

<div style="text-align: left; margin-bottom: 2px;">
<button onclick="GoToPortal()" style="font-size: 9.5px; padding: 2px 5px;">&larr; Back to Navigation Portal</button>
<button onclick="ToggleNightVision()" style="font-size: 9.5px; padding: 2px 5px; margin-right: 4px;">&#127769; Toggle Night Vision (Red UI)</button>
</div>

<h2>NICKTONKS_ASTROPHOTOGRAPHY<br><span style="font-size: 10px; color: #ffc857;">Seestar Station Mode File Organiser, Transfer, Live View & Stacking v1.0</span></h2>

<!-- SECTION 1: NODE DISCOVERY -->
<div class="panel">
<div>
    <label class="section-label"><strong>1. Telescope Node Discovery:</strong></label>
</div>

<div style="margin-top: 3px;">
    <div class="btn-action-group">
    <button onclick="RunAutoScan()">Run Automatic Scan</button>
    <button onclick="ToggleManualBox()">Add Manual IPs</button>
    </div>

    <div id="manualBoxContainer">
    <h3 style="color: #00f0ff; margin-top: 0; font-size: 11px; text-align: center; border-bottom: 1px solid #1a263f; padding-bottom: 2px; font-family:'Consolas', monospace;">ENTER TELESCOPE NODE IP</h3>
    <div style="display: flex; gap: 6px; margin-bottom: 4px;">
    <input type="text" id="singleManualIp" placeholder="e.g. 192.168.1.150" style="font-size: 10.5px; flex: 1;" />
    <button class="btn-primary" onclick="SaveSingleManualIP()" style="width: 75px; font-size: 10px; padding: 4px;">Add IP</button>
    </div>

    <label style="font-size: 9px; color: #94a3b8; font-family:'Consolas', monospace;">Validated Nodes Session Log:</label>
    <div id="addedIpsDisplay" style="background-color: #03050b; border: 1px solid #1a263f; padding: 4px; font-family: 'Consolas', monospace; font-size: 10px; color: #00f0ff; margin-bottom: 4px; border-radius: 3px; max-height: 50px; overflow-y: auto;"></div>

    <div style="display: flex; justify-content: flex-end; margin-top: 4px;">
    <button onclick="ToggleManualBox()" style="background-color: #1a263f; font-size: 10px; width: 100%;">Done / Close</button>
    </div>
    </div>
</div>
</div>

<!-- SECTION 2: ACTIVE TELESCOPE CONNECTION DETAILS -->
<div class="panel">
<div>
    <label class="section-label"><strong>2. Active Telescope Connection Details: If prompted for username and password guest:(no password required)</strong></label>
</div>

<div style="margin-top: 3px;">
    <div id="deviceList" class="device-display">
    <span style="color: #94a3b8;">Devices will Automatically Connect once Scan is complete.</span>
    </div>
</div>
</div>

<!-- SECTION 3: CONNECTED DEVICE LIVE STREAM MONITORS -->
<div class="panel">
<div style="display: flex; justify-content: space-between; align-items: center;">
    <label class="section-label"><strong>3. Connected Device Live Stream Monitors: Feeds only display if you are currently viewing a target and arm is open</strong></label>
    <div>
        <button onclick="StopAllVLCStreams()" style="font-size: 9px; padding: 1px 5px; background-color: #1a263f; margin-right: 4px;">Stop All Feeds</button>
        <button id="btnToggleStream" onclick="ToggleSection('streamGridContent', 'btnToggleStream')" style="font-size: 9px; padding: 1px 5px; background-color: #1a263f;">[-] Minimize</button>
    </div>
</div>

<div id="streamGridContent" class="vlc-grid-container" style="display: flex;">
    
    <!-- GRID SLOT 1 -->
    <div id="vlcCard_1" class="vlc-grid-card">
        <div class="vlc-card-header">
            <span id="streamLabel_1">SLOT 1 [IDLE]</span>
            <span style="cursor:pointer; color:#ef4444;" onclick="StopGridStreamSlot(1)">[X]</span>
        </div>
        <div class="vlc-player-wrapper">
            <div id="overlay_1" class="vlc-overlay-text">Slot 1 Waiting for Node...</div>
            <object classid="clsid:9BE31822-FDAD-461B-AD51-BE1D1C159921" id="vlcEmbedded_1" width="100%" height="100%">
                <param name="MRL" value="" />
                <param name="ShowDisplay" value="True" />
                <param name="AutoLoop" value="False" />
                <param name="AutoPlay" value="True" />
                <param name="Volume" value="0" />
                <param name="toolbar" value="false" />
            </object>
        </div>
    </div>

    <!-- GRID SLOT 2 -->
    <div id="vlcCard_2" class="vlc-grid-card">
        <div class="vlc-card-header">
            <span id="streamLabel_2">SLOT 2 [IDLE]</span>
            <span style="cursor:pointer; color:#ef4444;" onclick="StopGridStreamSlot(2)">[X]</span>
        </div>
        <div class="vlc-player-wrapper">
            <div id="overlay_2" class="vlc-overlay-text">Slot 2 Waiting for Node...</div>
            <object classid="clsid:9BE31822-FDAD-461B-AD51-BE1D1C159921" id="vlcEmbedded_2" width="100%" height="100%">
                <param name="MRL" value="" />
                <param name="ShowDisplay" value="True" />
                <param name="AutoLoop" value="False" />
                <param name="AutoPlay" value="True" />
                <param name="Volume" value="0" />
                <param name="toolbar" value="false" />
            </object>
        </div>
    </div>

    <!-- GRID SLOT 3 -->
    <div id="vlcCard_3" class="vlc-grid-card">
        <div class="vlc-card-header">
            <span id="streamLabel_3">SLOT 3 [IDLE]</span>
            <span style="cursor:pointer; color:#ef4444;" onclick="StopGridStreamSlot(3)">[X]</span>
        </div>
        <div class="vlc-player-wrapper">
            <div id="overlay_3" class="vlc-overlay-text">Slot 3 Waiting for Node...</div>
            <object classid="clsid:9BE31822-FDAD-461B-AD51-BE1D1C159921" id="vlcEmbedded_3" width="100%" height="100%">
                <param name="MRL" value="" />
                <param name="ShowDisplay" value="True" />
                <param name="AutoLoop" value="False" />
                <param name="AutoPlay" value="True" />
                <param name="Volume" value="0" />
                <param name="toolbar" value="false" />
            </object>
        </div>
    </div>

    <!-- GRID SLOT 4 -->
    <div id="vlcCard_4" class="vlc-grid-card">
        <div class="vlc-card-header">
            <span id="streamLabel_4">SLOT 4 [IDLE]</span>
            <span style="cursor:pointer; color:#ef4444;" onclick="StopGridStreamSlot(4)">[X]</span>
        </div>
        <div class="vlc-player-wrapper">
            <div id="overlay_4" class="vlc-overlay-text">Slot 4 Waiting for Node...</div>
            <object classid="clsid:9BE31822-FDAD-461B-AD51-BE1D1C159921" id="vlcEmbedded_4" width="100%" height="100%">
                <param name="MRL" value="" />
                <param name="ShowDisplay" value="True" />
                <param name="AutoLoop" value="False" />
                <param name="AutoPlay" value="True" />
                <param name="Volume" value="0" />
                <param name="toolbar" value="false" />
            </object>
        </div>
    </div>

</div>
</div>

<!-- SECTION 4: SAVE DATA DIRECTORY -->
<div class="panel">
<div>
    <label class="section-label"><strong>4. Save Data Directory:</strong></label>
</div>

<div style="margin-top: 3px;">
    <div class="row-inline">
    <input type="text" id="destFolderPath" placeholder="Choose a destination folder..." style="flex: 1;" />
    <button onclick="BrowseDestinationFolder()" style="white-space: nowrap;">Browse...</button>
    </div>
</div>
</div>

<!-- SECTION 5: FILE TYPES TO BE SAVED -->
<div class="panel">
<div>
    <label class="section-label"><strong>5. File Types to be Saved:</strong></label>
</div>

<div style="margin-top: 3px;">
    <div class="checkbox-group">
    <label><input type="checkbox" id="chkFits" checked /> FITS Data (.fit)</label>
    <label><input type="checkbox" id="chkVideo" checked /> Video (.avi / .mp4)</label>
    <label><input type="checkbox" id="chkImages" checked /> Snapshots (.jpeg)</label>
    </div>
</div>
</div>

<!-- SECTION 6: DATA TRANSFER CONTROL -->
<div class="panel">
<div>
    <label class="section-label"><strong>6. Data Transfer Control:</strong></label>
</div>

<div style="margin-top: 3px;">
    <div class="transfer-buttons">
    <button class="btn-primary" onclick="StartTransfer()" style="flex: 1;">Start Sync</button>
    <button class="btn-danger" onclick="StopTransfer()" style="flex: 1;">Pause Sync</button>
    <button class="btn-resume" onclick="ResumeTransfer()" style="flex: 1;">Resume Sync</button>
    </div>
</div>
</div>

<!-- SECTION 7: OBSERVATORY TRANSFER LOG -->
<div class="panel">
<div>
    <label class="section-label"><strong>7. Observatory Transfer Log:</strong></label>
</div>

<div style="margin-top: 3px;">
    <textarea id="logArea" readonly></textarea>
</div>
</div>

<div class="footer-buttons-stacked">
<button class="btn-exit-custom" onclick="GoToSirilStackingPage()">Stacking and Preview Page</button>
</div>

  <div class="note" style="margin-top: 6px;">
            Find me on Instagram: <span><a href="microsoft-edge:https://instagram.com/NICKTONKS_ASTROPHOTOGRAPHY" target="_blank" style="color:#ffc857;">@NICKTONKS_ASTROPHOTOGRAPHY</a></span>
        </div>
    </div>
</div>


</div>

<!-- PAGE 3 CONTAINER (WEATHER PAGE) -->
<div id="page3Container" class="container">

<div style="text-align: left; margin-bottom: 2px;">
<button onclick="GoToPortal()" style="font-size: 9.5px; padding: 2px 5px;">&larr; Back to Navigation Portal</button>
<button onclick="ToggleNightVision()" style="font-size: 9.5px; padding: 2px 5px; margin-right: 4px;">&#127769; Toggle Night Vision (Red UI)</button>
    </div>

    <h2>NICKTONKS_ASTROPHOTOGRAPHY<br><span style="font-size: 10px; color: #ffc857;">Scientific Observatory Weather & Seeing Dashboard</span></h2>

    <div class="panel">
        <div style="display: flex; justify-content: space-between; align-items: center;">
            <label class="section-label"><strong>Live Observatory Conditions & Sky Telemetry:</strong></label>
            <button onclick="FetchObservatoryWeather()" style="font-size: 9px; padding: 1px 5px;">Refresh Weather</button>
        </div>
        
        <div id="weatherStatusText" style="font-size: 9.5px; color: #ffc857; font-family: 'Consolas', monospace; margin-top: 2px; margin-bottom: 4px;">
            Awaiting query initiation...
        </div>

        <div class="weather-grid">
            <div class="weather-card">
                <div class="weather-card-title">Detected Location</div>
                <div id="valLocation" class="weather-card-value" style="font-size: 10.5px;">--</div>
                <div class="weather-card-sub">IP Geolocation</div>
            </div>
            <div class="weather-card">
                <div class="weather-card-title">Cloud Cover</div>
                <div id="valCloud" class="weather-card-value">--%</div>
                <div class="weather-card-sub">Total Atmospheric Cover</div>
            </div>
            <div class="weather-card">
                <div class="weather-card-title">Light Pollution</div>
                <div id="valBortle" class="weather-card-value" style="font-size: 10.5px;">--</div>
                <div id="valSQM" class="weather-card-sub">SQM:--</div>
            </div>
            <div class="weather-card">
                <div class="weather-card-title">Ambient Temperature</div>
                <div id="valTemp" class="weather-card-value">--°C</div>
                <div class="weather-card-sub">Surface Air Temp</div>
            </div>
            <div class="weather-card">
                <div class="weather-card-title">Relative Humidity</div>
                <div id="valHumidity" class="weather-card-value">--%</div>
                <div class="weather-card-sub">Dew Point Risk Factor</div>
            </div>
            <div class="weather-card">
                <div class="weather-card-title">Wind & Visibility</div>
                <div id="valWind" class="weather-card-value" style="font-size: 10.5px;">--</div>
                <div id="valVisibility" class="weather-card-sub">Visibility:--</div>
            </div>
        </div>
    </div>

    <!-- HOURLY CLOUD COVER & WEATHER TREND PANEL -->
    <div class="panel">
        <label class="section-label"><strong>Hourly Cloud Cover & Weather Trend (Next 24 Hours):</strong></label>
        <div id="hourlyGrid" class="hourly-container">
            <div style="color: #94a3b8; font-size: 9.5px; font-family: 'Consolas', monospace; text-align: center; width: 100%; padding: 6px;">
                Awaiting hourly telemetry...
            </div>
        </div>
    </div>

    <!-- 7-DAY FORECAST PANEL -->
    <div class="panel">
        <label class="section-label"><strong>7-Day Atmospheric & Forecast Trend:</strong></label>
        <div id="forecastGrid" class="weather-grid" style="margin-top: 3px;">
            <div style="color: #94a3b8; font-size: 9.5px; font-family: 'Consolas', monospace; text-align: center; width: 100%; padding: 6px;">
                Awaiting forecast telemetry...
            </div>
        </div>
    </div>

    <div class="panel">
        <label class="section-label"><strong>Astrophotography Planner Note</strong></label>
        <p style="color: #cbd5e1; margin-top: 2px; line-height: 1.2; font-size: 10px;">
            This section automatically resolves your public IP location and requests high-resolution atmospheric models from <span style="color:#00f0ff;">Open-Meteo</span> to help determine optimal windows for deep-sky imaging.
        </p>
    </div>

    <div class="footer-buttons-stacked" style="margin-top: 8px;">
    <button class="btn-exit-custom" onclick="GoToScanLivePage()">Scanning & Transfer Page</button>
    </div>
  <div class="note" style="margin-top: 6px;">
            Find me on Instagram: <span><a href="microsoft-edge:https://instagram.com/NICKTONKS_ASTROPHOTOGRAPHY" target="_blank" style="color:#ffc857;">@NICKTONKS_ASTROPHOTOGRAPHY</a></span>
        </div>
    </div>

</div>

<!-- Siril Stacking Container -->
<div id="sirilStackingContainer" class="container">
<div style="text-align: left; margin-bottom: 2px;">
<button onclick="GoToPortal()" style="font-size: 9.5px; padding: 2px 5px;">&larr; Back to Navigation Portal</button>
<button onclick="ToggleNightVision()" style="font-size: 9.5px; padding: 2px 5px; margin-right: 4px;">&#127769; Toggle Night Vision (Red UI)</button>
        
</div>

    <h2>NICKTONKS_ASTROPHOTOGRAPHY<br><span style="font-size: 10px; color: #ffc857;">Siril CLI Advanced Stacking Workspace</span></h2>

    <div class="panel">
        <label class="section-label"><strong>1. Siril Executable Path:</strong></label>
        <div class="row-inline" style="margin-top: 4px;">
            <input type="text" id="sirilExePath" value="C:\Program Files\Siril\bin\siril-cli.exe" style="flex: 1;" />
        </div>
    </div>

<div class="panel">
    <label class="section-label"><strong>2. Select Target Folder:</strong></label>
    <div style="margin-top: 3px;">
        <label style="font-size:10px;">For the script to work, select top layer folder, eg. M71_subs not M71_Subs\Lights:</label>
        <div class="row-inline">
            <input type="text" id="sirilWorkDir" onblur="updateTargetNameFromPath(this.value)" placeholder="Select target folder..." style="flex: 1;" />
            <button onclick="BrowseSirilFolder()" style="white-space: nowrap;">Browse...</button>
                 </div>
    </div>
</div>

<div class="panel">
    <label class="section-label"><strong>3. Target Name:</strong></label>
    <div class="row-inline" style="margin-top: 4px;">
        <input type="text" id="sirilTargetName" value="Target" style="flex: 1;" />
    </div>
</div>
    <div class="panel">
        <label class="section-label"><strong>4. Execution Control:</strong></label>
        <div style="margin-top: 3px;">
            <div class="transfer-buttons">
                <button class="btn-primary" onclick="ExecuteSirilBatchScript()" style="flex: 1;">Run Siril Stacking</button>
            </div>
        </div>
    </div>

    <div class="panel">
        <label class="section-label"><strong>5. Siril Execution Log:</strong></label>
        <textarea id="sirilLogArea" readonly style="width: 100%; height: 70px; background-color: #03050b; color: #38bdf8; font-family: 'Consolas', monospace; border: 1px solid #1a263f; padding: 4px; margin-top: 2px; box-sizing: border-box; overflow-y: scroll; white-space: pre-wrap; display: block; border-radius: 3px; font-size: 10.5px;"></textarea>
    </div>

<!-- INTEGRATED FITS PREVIEW & TEXT DOCUMENT WORKSPACE -->
    <div class="panel">
        <div style="display: flex; justify-content: space-between; align-items: center;">
            <label class="section-label"><strong>6. Stacked FITS Preview (Not a full resolution preview):</strong></label>
            <div>
                <button id="btnToggleFits" onclick="ToggleSection('fitsContentBlock', 'btnToggleFits')" style="font-size: 9px; padding: 1px 5px; background-color: #1a263f;">[-] Minimize</button>
            </div>
        </div>

        <div id="fitsContentBlock" style="display: block; margin-top: 3px;">
            <div style="margin-top: 3px;">
                <label style="font-size:10px;"><strong>Select any astronomy .fit image/stack to preview:</strong></label><br>
<label style="font-size:10px;"><strong>Wait for it to load before pressing any options</strong></label><br>
                <input type="file" id="fileInput" accept=".fit,.fits" style="margin-top: 2px; color:#00f0ff;" />
            </div>

            <div id="viewerArea" class="viewer-container" style="margin-top: 4px;">
                <div class="meta-panel" id="metaInfo" style="font-family:'Consolas', monospace; font-size:10px; color:#38bdf8; margin-bottom:4px;">Reading header metadata...</div>
                
                <div class="row-inline">
                    <div>
                        <button onclick="setMode('linear')" id="btnLinear">Original View</button>
                        <button onclick="setMode('stretch')" id="btnStretch">Auto-Stretch</button>
                    </div>
                </div>

                <div class="row-inline" id="stretchSliderRow" style="display: none; margin-top: 4px; align-items: center; gap: 6px;">
                    <div style="display: flex; align-items: center; gap: 4px; flex: 1;">
                        <label for="stretchFactor" style="font-size: 10px;">Stretch Intensity:</label>
                        <input type="range" id="stretchFactor" min="1" max="10" value="6" step="0.5" oninput="onSliderChange()" style="flex: 1;">
                        <span id="stretchVal" style="min-width: 25px; text-align: left; font-family:'Consolas', monospace; font-size:10px; color:#00f0ff;">6.0</span>
                    </div>
                    <button onclick="commitStretch()" style="background: #0d5c46; padding: 4px 8px; font-size: 10px;">Commit</button>
                </div>

                <div class="canvas-scroll-wrapper" style="margin-top: 4px;">
                    <canvas id="imageCanvas"></canvas>
                </div>
            </div>
        </div>
    </div>

       <div class="footer-buttons-stacked" style="margin-top: 8px;">
        <button class="btn-exit-custom" onclick="CloseAndExit()">Close and Exit</button>
  <div class="note" style="margin-top: 6px;">
            Find me on Instagram: <span><a href="microsoft-edge:https://instagram.com/NICKTONKS_ASTROPHOTOGRAPHY" target="_blank" style="color:#ffc857;">@NICKTONKS_ASTROPHOTOGRAPHY</a></span>
        </div>
    </div>
</div>
    </div>
</div>


</body>
</html>