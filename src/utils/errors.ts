export function getErrorMessage(t: (key: string, params?: any, error?: any) => string, error: any): string {
    if (!error) return t('errors.unknownError')

    if (error === 'GameDirectoryNotSet') return t('errors.gameDirectoryNotSet')
    if (error === 'InvalidName')         return t('errors.invalidName')
    if (error === 'InvalidFormat')       return t('errors.invalidFormat')
    if (error === 'ModAlreadyExists')    return t('errors.modAlreadyExists')
    if (error === 'InvalidMod')          return t('errors.invalidMod')
    if (error === 'MultipleModsFound')   return t('errors.multipleModsFound')
    if (error === 'UnsupportedFormat')   return t('errors.unsupportedFormat')

    if (error === "SymlinkAdminRequired")       return t('errors.symlinkAdminRequired')

    if (error?.SyncMethodInvalid)        return t('errors.syncMethodInvalid', { method: error.SyncMethodInvalid })
    if (error?.PathNotFound)             return t('errors.pathNotFound', { path: error.PathNotFound.path })
    if (error?.UnknownError)             return t('errors.unknownError', { error: error.UnknownError })

    if (error?.SyncFailed) {
        const sync = error.SyncFailed
        if (sync?.type === 'SymlinkAdminRequired')       return t('errors.symlinkAdminRequired')
        if (sync?.type === 'ModPathNotFound')             return t('errors.modPathNotFound', { path: sync.details })
        if (sync?.type === 'CopyFailed')                 return t('errors.copyFailed', { error: sync.details })
        if (sync?.type === 'SymlinkFailed')              return t('errors.symlinkFailed', { error: sync.details })
        if (sync?.type === 'HardlinkFailed')             return t('errors.hardlinkFailed', { error: sync.details })
        if (sync?.type === 'DirectoryCreationFailed')    return t('errors.directoryCreationFailed', { error: sync.details })
        if (sync?.type === 'RemovalFailed')              return t('errors.removalFailed', { error: sync.details })
    }

    return t('errors.unknownError', { error: JSON.stringify(error) })
}