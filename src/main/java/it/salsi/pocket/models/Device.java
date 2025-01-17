/*
The MIT License (MIT)

Original Work Copyright (c) 2018-2025 Antonio Salsi

Permission is hereby granted, free of charge, to any person obtaining a copy of
this software and associated documentation files (the "Software"), to deal in
the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NON INFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR
COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER
IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

package it.salsi.pocket.models;

import com.fasterxml.jackson.annotation.JsonIgnore;
import jakarta.persistence.*;
import jakarta.validation.constraints.NotEmpty;
import jakarta.validation.constraints.NotNull;
import jakarta.validation.constraints.Size;
import lombok.Getter;
import lombok.Setter;
import lombok.ToString;

import java.time.Clock;
import java.time.Instant;
import java.util.UUID;

import static it.salsi.pocket.models.Device.Status.ACTIVE;

@ToString
//@Data no perchè crash OneToMany
@Entity(name = "devices")
@Table(
        indexes = {@Index(name = "idx_devices_device_serial", columnList = "deviceSerial"), @Index(name = "idx_devices_token", columnList = "token")},
        uniqueConstraints = {@UniqueConstraint(columnNames = {"token"})}
)
@Getter
@Setter
@SuppressWarnings("JpaDataSourceORMInspection")
public class Device {


    @SuppressWarnings("unused")
    public enum Status {
        NOT_ACTIVE, ACTIVE, DELETED, INVALIDATED
    }

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Long id = 0L;

    @Size(max = 256, message = "max size exceeded; maximum 256 char")
    @NotEmpty(message = "field empty")
    @NotNull(message = "field null")
    @Column(nullable = false)
    private String uuid = "";

    @JsonIgnore
    private String version;

    @JsonIgnore
    private String address;

    @Column(nullable = false)
    private Status status = ACTIVE;

    @Temporal(TemporalType.TIMESTAMP)
    private Long timestampLastUpdate = 0L;

    @Temporal(TemporalType.TIMESTAMP)
    @Column(nullable = false)
    private Long timestampLastLogin = 0L;

    @Temporal(TemporalType.TIMESTAMP)
    @Column(nullable = false)
    private Long timestampCreation = Instant.now(Clock.systemUTC()).getEpochSecond();

    @ToString.Exclude
    @JoinColumn(name = "user_id", referencedColumnName = "id")
    @ManyToOne(fetch = FetchType.LAZY)
    @JsonIgnore
    private User user;

    @JsonIgnore
    @Lob
    private String note;

    @JsonIgnore
    @Lob
    private String publicKey;

    @JsonIgnore
    @Lob
    private String privateKey;

    public void updateTimestampLastLogin() {
        timestampLastLogin = Instant.now(Clock.systemUTC()).getEpochSecond();
    }

    public void updateTimestampLastUpdate() {
        timestampLastUpdate = Instant.now(Clock.systemUTC()).getEpochSecond();
    }

    public Device() {}

    public Device(@org.jetbrains.annotations.NotNull final User user) {
        this();
        setUser(user);
        setUuid(UUID.randomUUID().toString());
        setStatus(Device.Status.ACTIVE);
        updateTimestampLastLogin();
        updateTimestampLastUpdate();
    }
}
