package it.salsi.pocket.services;

import lombok.extern.java.Log;
import org.jetbrains.annotations.NotNull;
import org.springframework.stereotype.Service;

import java.util.HashMap;
import java.util.Map;
import java.util.Optional;


@Log
@Service
public final class CacheManagerImpl implements CacheManager {

    @NotNull
    private final Map<String, CacheRecord> map = new HashMap<>();

    public CacheManagerImpl() {

    }

    public boolean add(@NotNull final CacheRecord record) {
        if(map.containsKey(record.uuid())) {
            return false;
        }
        map.put(record.uuid(), record);
        return true;
    }

    public @NotNull Optional<CacheRecord> get(@NotNull final CacheRecord record) {
        if(!map.containsKey(record.uuid())) {
            return Optional.empty();
        }
        return Optional.ofNullable(map.get(record.uuid()));
    }

    public boolean rm(@NotNull final CacheRecord record) {
        if(!map.containsKey(record.uuid())) {
            return false;
        }
        return map.remove(record.uuid()) != null;
    }

    public boolean has(@NotNull final CacheRecord record) {
        return  map.containsKey(record.uuid());
    }

}