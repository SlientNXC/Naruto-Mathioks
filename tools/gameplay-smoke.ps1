# Optional integration check against a separately launched Creative test world.
# Uses Horizon's documented local control API, without desktop input.
param(
    [Parameter(Mandatory=$true)][string]$CtlPath,
    [Parameter(Mandatory=$true)][string]$SocketPath,
    [string]$ExpectedWorld='NarutoPrototypeProof'
)
$ErrorActionPreference='Stop'

function Invoke-TestGame {
    param([string[]]$ControlArguments)
    $taskResponse=& $CtlPath --socket $SocketPath --timeout 45 @ControlArguments
    if($LASTEXITCODE -ne 0){throw ($taskResponse -join "`n")}
    $taskReply=($taskResponse -join "`n") | ConvertFrom-Json
    if($taskReply.error){throw $taskReply.error}
    return $taskReply
}

$null=Invoke-TestGame @('wait-ready','40')
$taskState=Invoke-TestGame @('state')
if($taskState.world -ne $ExpectedWorld -or $taskState.gamemode -ne 'creative'){
    throw 'Run this check only in the intended isolated Creative test world.'
}
if($taskState.screen -ne 'playing'){
    throw 'The test world must be open with no menu covering it.'
}

# This caught the 0.1.0 bug: an integrated server also has dedicated options.
$taskSince=(Invoke-TestGame @('events')).last
$null=Invoke-TestGame @('key','b','tap')
$null=Invoke-TestGame @('wait-for','chat','Chakra','--since',"$taskSince",'--timeout','10')
$taskMessages=Invoke-TestGame @('messages','12')
$taskStatus=@($taskMessages.chat | Where-Object {$_ -match '^Level \d+ \| Chakra \d+/\d+'})
if($taskStatus.Count -eq 0){
    throw ('Character creation/status failed: '+($taskMessages.chat -join ' | '))
}

[pscustomobject]@{
    check='native character creation through the B key'
    world=$taskState.world
    passed=$true
    status=$taskStatus[0]
} | ConvertTo-Json
